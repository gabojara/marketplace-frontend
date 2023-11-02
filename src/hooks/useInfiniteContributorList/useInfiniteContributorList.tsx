import { components } from "src/__generated/api";
import { ApiResourcePaths } from "src/hooks/useRestfulData/config";
import { useInfiniteRestfulData } from "src/hooks/useRestfulData/useRestfulData";

type QueryParam = {
  key: string;
  value: Array<string | number | boolean>;
};

interface UseInfiniteContributorsProps {
  projectId: string;
  queryParams?: QueryParam[];
}

export default function useInfiniteContributorList({ projectId, queryParams }: UseInfiniteContributorsProps) {
  return useInfiniteRestfulData<components["schemas"]["ContributorsPageResponse"]>(
    {
      resourcePath: ApiResourcePaths.GET_PROJECT_CONTRIBUTORS,
      pageSize: 15,
      pathParam: projectId,
      queryParams,
    },
    { queryKey: ["contributors", projectId, queryParams] }
  );
}
