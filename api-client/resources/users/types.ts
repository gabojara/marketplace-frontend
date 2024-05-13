import { components } from "src/__generated/api";

type PublicUserProfileResponse = components["schemas"]["PublicUserProfileResponseV2"];
type PublicUserLanguagesResponse = components["schemas"]["UserProfileLanguagePage"];
type PublicUserEcosystemsResponse = components["schemas"]["UserProfileEcosystemPage"];
type PublicUserStatsResponse = components["schemas"]["UserProfileStatsV2"];

export interface UserPublicProfileResponse extends PublicUserProfileResponse {}
export interface UserPublicLanguagesResponse extends PublicUserLanguagesResponse {}
export interface UserPublicEcosystemsResponse extends PublicUserEcosystemsResponse {}
export interface UserPublicStatsResponse extends PublicUserStatsResponse {}
