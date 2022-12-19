import githubLogo from "assets/img/github-logo.svg";
import telegramLogo from "assets/img/telegram-logo.svg";
import onlyDustLogo from "assets/img/onlydust-logo.png";
import CodeIcon from "assets/icons/Code";
import RemainingBudget from "../RemainingBudget";
import { MouseEvent } from "react";
import { useIntl } from "src/hooks/useIntl";
import { buildGithubLink } from "src/utils/stringUtils";

interface ProjectInformationProps {
  name: string;
  budget?: {
    remainingAmount: number;
    initialAmount: number;
  } | null;
  details?: {
    description?: string | null;
    telegramLink?: string | null;
  } | null;
  githubRepoInfo?: {
    owner?: string;
    name?: string;
    contributors?: { login: string }[];
  };
}

const linkClickHandlerFactory = (url: string) => (e: MouseEvent<HTMLDivElement>) => {
  e.preventDefault();
  window?.open(url, "_blank")?.focus();
};

export default function ProjectInformation({ name, details, budget, githubRepoInfo }: ProjectInformationProps) {
  return (
    <div className="flex flex-row divide-x divide-neutral-600 gap-5 justify-items-center font-walsheim">
      <div className="flex flex-col basis-4/12 gap-5 justify-around">
        <div className="flex flex-row gap-3 items-center">
          <div className="border-4 border-neutral-600 p-2 rounded-2xl">
            <img className="md:w-8 w-4 hover:opacity-90" src={onlyDustLogo} alt="Project Logo" />
          </div>
          <div className="flex flex-col">
            <div className="text-2xl font-bold">{name}</div>
            <div className="text-md text-neutral-500 font-bold">{details?.description}</div>
          </div>
        </div>
        <div className="flex flex-row border border-neutral-600 w-fit px-2 py-1 rounded-2xl gap-2">
          <div>
            <CodeIcon className="fill-gray-400" />
          </div>
          <div>Cairo</div>
        </div>
      </div>
      <div className="flex flex-col basis-8/12 pl-8 justify-around gap-5">
        <div>EVM interpreter written in Cairo, a sort of ZK-EVM emulator, leveraging STARK proof system.</div>
        <div className="flex flex-row divide-x divide-neutral-600">
          <div className="flex flex-row gap-3 pr-5">
            {githubRepoInfo?.owner && githubRepoInfo?.name && (
              <LinkWithLogo link={buildGithubLink(githubRepoInfo?.owner, githubRepoInfo?.name)} logo={githubLogo} />
            )}
            {details?.telegramLink && <LinkWithLogo link={details?.telegramLink} logo={telegramLogo} />}
          </div>
          <div className="flex flex-row justify-around items-center pl-8 gap-5">
            <div className="flex">301 contributors</div>
            <div className="flex">8 contributions</div>
          </div>
        </div>
      </div>
    </div>
  );
}

interface GithubDetailsProps extends React.PropsWithChildren {
  title: string;
}

function GithubDetails({ title, children }: GithubDetailsProps) {
  return (
    <div className="flex flex-col">
      <div className="flex text-lg font-bold">{title}</div>
      <div className="flex">{children}</div>
    </div>
  );
}

interface TelegramLinkProps {
  link: string;
  logo: string;
}

function LinkWithLogo({ link, logo }: TelegramLinkProps) {
  return (
    <div className="border-1 rounded-md p-2 grayscale bg-white border-slate-500 opacity-80 hover:opacity-50">
      <div onClick={linkClickHandlerFactory(link)}>
        <img className="md:w-6 w-3" src={logo} alt="Telegram Logo" />
      </div>
    </div>
  );
}

interface GithubRepoInfoProps {
  technology: string;
  leadContributor: string;
  numberOfContributors: number;
}

function GithubRepoInfo({ technology, leadContributor, numberOfContributors }: GithubRepoInfoProps) {
  const { T } = useIntl();
  return (
    <div className="flex flex-row justify-between items-center">
      <GithubDetails title={T("project.details.overview.technologies")}>{technology}</GithubDetails>
      <GithubDetails title={T("project.details.overview.projectLeader")}>{leadContributor}</GithubDetails>
      <GithubDetails title={T("project.details.overview.contributors")}>{numberOfContributors}</GithubDetails>
    </div>
  );
}
