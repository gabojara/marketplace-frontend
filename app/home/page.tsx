import { PageGrid } from "app/home/components/page-grid/page-grid";
import { Activity } from "app/home/features/activity/activity";
import { Journey } from "app/home/features/journey/journey";
import { JourneyGuard } from "app/home/features/journey/journey.guard";
import { LeadProjects } from "app/home/features/lead-projects/lead-projects";
import { Profile } from "app/home/features/profile/profile";
import { RecommendedProjects } from "app/home/features/recommended-projects/recommended-projects";
import { Rewards } from "app/home/features/rewards/rewards";
import { TrendyProjects } from "app/home/features/trendy-projects/trendy-projects";

import { RequiredAuthGuard, RequiredUnauthGuard } from "components/features/auth0/guards/auth-guard";
import { Container } from "components/layout/container/container";

export default function HomePage() {
  return (
    <Container>
      <PageGrid>
        <JourneyGuard>
          <Journey />
        </JourneyGuard>

        <RequiredAuthGuard>
          <Profile />
          <Rewards />
          <LeadProjects />
          <RecommendedProjects />
        </RequiredAuthGuard>

        <RequiredUnauthGuard>
          <TrendyProjects />
          <Activity />
        </RequiredUnauthGuard>
      </PageGrid>
    </Container>
  );
}
