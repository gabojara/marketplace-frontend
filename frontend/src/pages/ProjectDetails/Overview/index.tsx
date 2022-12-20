import Card from "src/components/Card";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { useIntl } from "src/hooks/useIntl";
import { buildGithubLink } from "src/utils/stringUtils";
import LeadContributor, { Contributor } from "src/components/LeadContributor";

type LanguageMap = { [languageName: string]: number }[];

interface OverviewProps {
  decodedReadme: string;
  githubRepoInfo?: {
    owner?: string;
    name?: string;
    contributors?: Contributor[];
    languages: LanguageMap;
  };
}

export default function Overview({ decodedReadme, githubRepoInfo }: OverviewProps) {
  const { T } = useIntl();
  return (
    <div className="flex flex-row items-start gap-5">
      <div className="flex w-3/4">
        <Card>
          <ReactMarkdown skipHtml={true} remarkPlugins={[[remarkGfm]]} className="prose lg:prose-xl prose-invert">
            {decodedReadme}
          </ReactMarkdown>
        </Card>
      </div>
      <div className="flex w-1/4">
        <Card>
          <div className="flex flex-col gap-3">
            {githubRepoInfo?.languages && (
              <OverviewPanelSection title={T("project.details.overview.technologies")}>{}</OverviewPanelSection>
            )}
            {githubRepoInfo?.contributors?.length && (
              <OverviewPanelSection title={T("project.details.overview.projectLeader")}>
                <LeadContributor {...githubRepoInfo?.contributors?.[0]} />
              </OverviewPanelSection>
            )}
            {githubRepoInfo?.contributors?.length && (
              <OverviewPanelSection title={T("project.details.overview.contributors")}>
                {githubRepoInfo.contributors.length}
              </OverviewPanelSection>
            )}
            {githubRepoInfo?.owner && githubRepoInfo?.name && (
              <OverviewPanelSection title={T("project.details.overview.githubLinkTitle")}>
                <a
                  href={buildGithubLink(githubRepoInfo.owner, githubRepoInfo.name)}
                  className="underline text-blue-600 hover:text-blue-800 visited:text-purple-600"
                >
                  {T("project.details.overview.githubLinkContent")}
                </a>
              </OverviewPanelSection>
            )}
          </div>
        </Card>
      </div>
    </div>
  );
}

interface OverviewPanelSectionProps extends React.PropsWithChildren {
  title: string;
}

function OverviewPanelSection({ title, children }: OverviewPanelSectionProps) {
  return (
    <div className="flex flex-col">
      <div className="flex text-lg font-bold">{title}</div>
      <div className="flex">{children}</div>
    </div>
  );
}
