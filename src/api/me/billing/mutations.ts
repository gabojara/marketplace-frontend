import { components } from "src/__generated/api";
import { API_PATH } from "src/api/ApiPath";
import MeApi from "src/api/me";
import { ME_BILLING_TAGS } from "src/api/me/billing/tags";
import { ME_TAGS } from "src/api/me/tags";
import { MeTypes } from "src/api/me/types";
import { UseMutationProps, useBaseMutation } from "src/api/useBaseMutation";

export type UseUpdateMeMeBody = components["schemas"]["PatchMeContract"];

const useUpdateBillingProfileType = ({
  options = {},
}: UseMutationProps<unknown, unknown, { type: MeTypes.billingProfileType }>) => {
  return useBaseMutation<{ type: MeTypes.billingProfileType }, unknown>({
    resourcePath: API_PATH.ME_BILLING_PROFILES,
    method: "PATCH",
    invalidatesTags: [
      { queryKey: MeApi.tags.user, exact: false },
      { queryKey: ME_BILLING_TAGS.allProfiles, exact: false },
      { queryKey: ME_TAGS.rewarded_pending_invoice(), exact: false },
    ],
    ...options,
  });
};

export default {
  useUpdateBillingProfileType,
};
