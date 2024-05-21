"use client";

import { useAuth0 } from "@auth0/auth0-react";

import { CommitteeApplicantPrivatePage } from "app/c/[committeeId]/applicant/features/private-page/private-page";
import { CommitteeApplicantPublicPage } from "app/c/[committeeId]/applicant/features/public-page/public-page";

export default function CommitteeApplicantPage() {
  const { isAuthenticated } = useAuth0();

  return (
    <div className="scrollbar-sm flex items-start justify-center">
      <div className="p-6">
        {isAuthenticated ? <CommitteeApplicantPrivatePage /> : <CommitteeApplicantPublicPage />}
      </div>
    </div>
  );
}
