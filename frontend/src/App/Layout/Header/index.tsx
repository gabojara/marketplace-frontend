import { gql } from "@apollo/client";
import { useLocation } from "react-router-dom";
import { RoutePaths } from "src/App";
import { useAuth } from "src/hooks/useAuth";
import { useIntl } from "src/hooks/useIntl";
import { SessionMethod, useSessionDispatch } from "src/hooks/useSession";
import { useGetPaymentRequestIdsQuery } from "src/__generated/graphql";
import View from "./View";
import { useImpersonationClaims } from "src/hooks/useImpersonationClaims";

export default function Header() {
  const location = useLocation();
  const { isLoggedIn, githubUserId } = useAuth();
  const { T } = useIntl();
  const dispatchSession = useSessionDispatch();
  const { impersonationSet } = useImpersonationClaims();
  const impersonating = !!impersonationSet;

  const { data: paymentRequestIdsQueryData } = useGetPaymentRequestIdsQuery({
    variables: { githubUserId },
    skip: !githubUserId,
  });
  const hasPayments =
    paymentRequestIdsQueryData?.paymentRequests && paymentRequestIdsQueryData.paymentRequests.length > 0;

  const myContributionsMenuItem = hasPayments ? T("navbar.payments") : undefined;
  const projectsMenuItem = myContributionsMenuItem ? T("navbar.projects") : undefined;

  return (
    <View
      menuItems={{
        [RoutePaths.Projects]: projectsMenuItem,
        [RoutePaths.Payments]: myContributionsMenuItem,
      }}
      isLoggedIn={isLoggedIn}
      selectedMenuItem={location.pathname}
      onLogin={() => dispatchSession({ method: SessionMethod.SetVisitedPageBeforeLogin, value: location.pathname })}
      impersonating={impersonating}
    />
  );
}

gql`
  query GetPaymentRequestIds($githubUserId: bigint!) {
    paymentRequests(where: { recipientId: { _eq: $githubUserId } }) {
      id
    }
  }
`;
