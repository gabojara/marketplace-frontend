import type { DefaultError } from "@tanstack/query-core";
import { useMutation } from "@tanstack/react-query";
import { useReactQueryAdapter } from "api-client/adapter/react-query/react-query-adapter";

import { PROJECT_TAGS } from "src/api/Project/tags";

import { postMyApplication } from "../fetch";
import { PostProjectApplicationCreateRequest } from "../types";

export function usePostMyApplication({ projectId }: { projectId: string }) {
  const { mutation } = useReactQueryAdapter(postMyApplication(), {
    invalidatesTags: [{ queryKey: PROJECT_TAGS.good_first_issues(projectId), exact: false }],
  });

  return useMutation<unknown, DefaultError, PostProjectApplicationCreateRequest>(mutation);
}
