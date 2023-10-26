import { Outlet, useOutletContext } from "react-router-dom";
import { useRestfulData } from "src/hooks/useRestfulData/useRestfulData";

import { Project } from "src/types";

type OutletContext = {
  project: Project;
};

export default function Rewards() {
  const { project } = useOutletContext<OutletContext>();
  const { id: projectId, slug: projectKey } = project;

  const { data: projectBudget, isLoading: isBudgetLoading } = useRestfulData({
    resourcePath: `/api/v1/projects/${projectId}/budgets`,
    method: "GET",
  });

  return (
    <Outlet
      context={{
        isBudgetLoading,
        projectBudget,
        projectId,
        projectKey,
      }}
    />
  );
}
