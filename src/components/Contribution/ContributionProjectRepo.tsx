import { useStackProjectOverview } from "src/App/Stacks/Stacks";
import RoundedImage, { ImageSize, Rounding } from "src/components/RoundedImage";
import { ShortProject, ShortRepo } from "src/types";

import { Link } from "components/ds/link/link";
import { Typography } from "components/layout/typography/typography";

export function ContributionProjectRepo({
  project,
  repo,
  truncateLength = 0,
}: {
  project: ShortProject;
  repo: ShortRepo;
  truncateLength?: number;
}) {
  const [openProjectOverview] = useStackProjectOverview();

  const onClickProject = () => {
    openProjectOverview({ slug: project.slug });
  };

  return (
    <div className="flex items-center gap-3">
      <RoundedImage
        src={project.logoUrl}
        alt={project.name}
        rounding={Rounding.Corners}
        size={ImageSize.Sm}
        useLogoFallback
      />

      <div className="w-full">
        <Link.Button onClick={onClickProject} className="whitespace-normal text-left" title={project.name}>
          {truncateLength && project.name.length > truncateLength
            ? project.name.substring(0, truncateLength) + "..."
            : project.name}
        </Link.Button>
        &nbsp;
        <Typography variant="body-s" as="span" className="text-spaceBlue-300">
          /
        </Typography>
        &nbsp;
        <Link href={repo.htmlUrl} className="whitespace-normal text-left" title={repo.name}>
          {truncateLength && repo.name.length > truncateLength
            ? repo.name.substring(0, truncateLength) + "..."
            : repo.name}
        </Link>
      </div>
    </div>
  );
}
