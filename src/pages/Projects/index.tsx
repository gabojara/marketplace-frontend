import Background, { BackgroundRoundedBorders } from "src/components/Background";
import { useAuth } from "src/hooks/useAuth";
import AllProjectsParent, { DEFAULT_SORTING } from "./AllProjects";
import FilterPanel from "./FilterPanel";
import { ProjectFilterProvider } from "./useProjectFilter";
import useScrollRestoration from "./AllProjects/useScrollRestoration";
import { Suspense, useEffect, useState } from "react";
import SearchBar from "./SearchBar";
import { useDebounce } from "usehooks-ts";
import SidePanel from "src/components/SidePanel";
import { SortingPanel } from "./Sorting/SortingPanel";
import { useLocalStorage } from "react-use";
import SEO from "src/components/SEO";
import AllProjectLoading from "./AllProjects/AllProjectsLoading";

export enum Sorting {
  Trending = "trending",
  ProjectName = "projectName",
  ReposCount = "reposCount",
  ContributorsCount = "contributorsCount",
}

export const PROJECT_SORTINGS = [Sorting.Trending, Sorting.ProjectName, Sorting.ReposCount, Sorting.ContributorsCount];

export default function Projects() {
  const { ledProjectIds } = useAuth();
  const isProjectLeader = !!ledProjectIds.length;

  const [searchQuery, setSearchQuery] = useState<string>("");
  const [search, setSearch] = useState<string>("");
  const debouncedSearchQuery = useDebounce<string>(search, 200);
  useEffect(() => setSearchQuery(debouncedSearchQuery), [debouncedSearchQuery]);

  const [sorting, setSorting] = useLocalStorage("PROJECT_SORTING_2", DEFAULT_SORTING);

  const [filterPanelOpen, setFilterPanelOpen] = useState(false);
  const [sortingPanelOpen, setSortingPanelOpen] = useState(false);

  const [technologies, setTechnologies] = useState<string[]>([]);
  const [sponsors, setSponsors] = useState<string[]>([]);

  const [loading, setLoading] = useState(false);

  const { ref, restoreScroll } = useScrollRestoration();

  return (
    <ProjectFilterProvider>
      <SEO />
      <Background ref={ref} roundedBorders={BackgroundRoundedBorders.Full}>
        <div className="flex max-w-7xl flex-col gap-6 px-4 py-4 md:mx-auto md:px-12 xl:pb-8 xl:pt-12">
          <div>
            <SearchBar search={search} setSearch={setSearch} />
          </div>
          <div className="flex h-full gap-6">
            <div className="sticky top-0 hidden shrink-0 basis-80 xl:block">
              <FilterPanel isProjectLeader={isProjectLeader} technologies={technologies} sponsors={sponsors} />
            </div>
            <div className="min-w-0 grow">
              {/* TODO(Backend): This is a temporary solution until we delete graphql Query
              At this moment we wont use the double loading anymore */}
              {loading && <AllProjectLoading />}
              <Suspense fallback={<AllProjectLoading />}>
                <AllProjectsParent
                  search={searchQuery}
                  clearSearch={() => setSearch("")}
                  sorting={sorting}
                  setSorting={setSorting}
                  restoreScroll={restoreScroll}
                  filterPanelOpen={filterPanelOpen}
                  setFilterPanelOpen={setFilterPanelOpen}
                  sortingPanelOpen={sortingPanelOpen}
                  setSortingPanelOpen={setSortingPanelOpen}
                  setTechnologies={setTechnologies}
                  setSponsors={setSponsors}
                  setLoading={setLoading}
                />
              </Suspense>
            </div>
          </div>
        </div>
      </Background>
      <SidePanel withBackdrop open={filterPanelOpen} setOpen={setFilterPanelOpen} placement="bottom">
        <FilterPanel isProjectLeader={isProjectLeader} fromSidePanel technologies={technologies} sponsors={sponsors} />
      </SidePanel>
      <SidePanel withBackdrop open={sortingPanelOpen} setOpen={setSortingPanelOpen} placement="bottom">
        <SortingPanel all={PROJECT_SORTINGS} current={sorting || DEFAULT_SORTING} onChange={setSorting} />
      </SidePanel>
    </ProjectFilterProvider>
  );
}
