import { escapeRegExp, filter } from "lodash";
import { ContributionFragment, WorkItemType } from "src/__generated/graphql";

type Props = {
  pattern?: string;
  contributions: ContributionFragment[];
};

export default function useFilteredContributions({ pattern = "", contributions }: Props) {
  const searchRegExps = pattern
    .split(" ")
    .map(str => str.trim())
    .filter(str => str.length > 0)
    .map(str => new RegExp(escapeRegExp(str), "i"));

  return filter(contributions, (contribution: ContributionFragment) => {
    if (!contribution.id) return;

    const types = {
      [WorkItemType.Issue]: contribution.githubIssue,
      [WorkItemType.PullRequest]: contribution.githubPullRequest,
      [WorkItemType.CodeReview]: contribution.githubCodeReview?.githubPullRequest,
    };

    const contributionType = types[contribution?.type as WorkItemType];

    const contributionFullText = contributionType?.number?.toString() + " " + contributionType?.title;
    return searchRegExps.filter(regexp => !regexp.test(contributionFullText)).length === 0;
  });
}
