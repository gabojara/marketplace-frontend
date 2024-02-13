import isDefined from "src/utils/isDefined";

import { AvatarGroup } from "components/ds/avatar-group/avatar-group";
import { Tooltip } from "components/ds/tooltip/tooltip";
import { Contributor } from "components/features/contributor/contributor";

import { TContributorsAvatars } from "./contributors-avatars.types";

export function ContributorsAvatars({ contributors, avatarProps }: TContributorsAvatars.Props) {
  function contributorsContent() {
    return (
      <div className="flex flex-row flex-wrap items-center gap-4 text-snow">
        {contributors
          .filter(
            contributor =>
              isDefined(contributor.githubUserId) && isDefined(contributor.login) && isDefined(contributor.avatarUrl)
          )
          .map(contributor => (
            <Contributor
              key={contributor.githubUserId}
              githubUserId={contributor.githubUserId}
              login={contributor.login}
              avatarUrl={contributor.avatarUrl}
              isRegistered={false}
              clickable
            />
          ))}
      </div>
    );
  }

  return (
    <Tooltip content={contributorsContent()}>
      <AvatarGroup
        avatars={contributors.map(contributor => ({
          src: contributor.avatarUrl,
          alt: contributor.login || "",
        }))}
        avatarProps={avatarProps}
      />
    </Tooltip>
  );
}
