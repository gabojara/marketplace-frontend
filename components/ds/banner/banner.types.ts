import { PropsWithChildren, ReactElement } from "react";
import { VariantProps } from "tailwind-variants";

import { TButton } from "components/ds/button/button.types";
import { TIcon } from "components/layout/icon/icon.types";

import { bannerVariants } from "./banner.variants";

export namespace TBanner {
  export type Variants = VariantProps<typeof bannerVariants>;

  export interface Props extends PropsWithChildren, Variants {
    title: string;
    description?: string;
    icon?: TIcon.Props;
    button?: TButton.Props;
    customButton?: ReactElement;
  }
}
