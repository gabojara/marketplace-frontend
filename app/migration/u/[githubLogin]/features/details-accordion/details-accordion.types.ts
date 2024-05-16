import { PropsWithChildren } from "react";

import { TDotsStatus } from "app/migration/u/[githubLogin]/components/dots-status/dots-status.types";

import { RemixIconsName } from "components/layout/icon/remix-icon-names.types";

import { Key } from "hooks/translate/use-translate";

export namespace TDetailsAccordion {
  interface Project {
    name: string;
    slug: string;
    avatarUrl: string;
    hasMissingGithubAppInstallation: boolean;
    hasPendingInvitation: boolean;
  }

  export interface Detail {
    name: string;
    avatarUrl: string;
    rankStatus: TDotsStatus.Props["status"];
    contributionCount: number;
    projectsCount: number;
    rewardsCount: number;
    earnedUsdAmount: number;
    projects: Project[];
  }

  export interface ItemInfoProps {
    icon: RemixIconsName;
    count: number;
    labelToken: Key;
  }
  export interface ProjectAvatarProps extends Project {}
  export interface StartContentProps extends Omit<Detail, "contributions"> {}
  export interface AccordionProps extends PropsWithChildren {
    details: Detail[];
  }
}
