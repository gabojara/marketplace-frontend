import { Menu, Transition } from "@headlessui/react";
import classNames from "classnames";
import { Fragment, PropsWithChildren, useState } from "react";
import Dot from "src/assets/icons/Dot";
import { withTooltip } from "src/components/Tooltip";
import { useIntl } from "src/hooks/useIntl";
import ErrorWarningLine from "src/icons/ErrorWarningLine";
import LogoutBoxRLine from "src/icons/LogoutBoxRLine";
import MoneyDollarCircleLine from "src/icons/MoneyDollarCircleLine";
import PayoutInfoSidePanel from "./PayoutInfoSidePanel";
import User3Line from "src/icons/User3Line";
import ContributorProfileSidePanel from "./ContributorProfileSidePanel";

type Props = {
  avatarUrl: string | null;
  login: string;
  logout: () => void;
  payoutSettingsInvalid: boolean;
  githubUserId?: number;
};

const View = ({ githubUserId, avatarUrl, login, logout, payoutSettingsInvalid }: Props) => {
  const { T } = useIntl();

  const [menuItemsVisible, setMenuItemsVisible] = useState(false);
  const [tooltipVisible, setTooltipVisible] = useState(false);
  const [payoutInfoSidePanelOpen, setPayoutInfoSidePanelOpen] = useState(false);
  const [profileSidePanelOpen, setProfileSidePanelOpen] = useState(false);

  return (
    <div className="relative">
      <Menu as="div" className="relative inline-block">
        <div>
          <Menu.Button
            id="profile-button"
            onMouseEnter={() => setTooltipVisible(true)}
            onMouseLeave={() => setTooltipVisible(false)}
            className={classNames(
              "flex gap-2 m-0.5 justify-center outline outline-1 px-2 py-1.5 items-center rounded-full text-sm font-belwe hover:bg-noise-medium hover:outline-2 ui-open:bg-noise-medium ui-open:outline-2",
              {
                "outline-greyscale-50/12": !payoutSettingsInvalid,
                "outline-orange-500": payoutSettingsInvalid,
              }
            )}
            data-testid="profile-button"
            {...withTooltip(T("profile.button.payoutSettingsInvalidTooltip"), {
              visible: payoutSettingsInvalid && tooltipVisible && !menuItemsVisible,
            })}
          >
            {avatarUrl && <img className="w-8 rounded-full" src={avatarUrl} />}
            <div className={classNames({ "mr-1": !payoutSettingsInvalid })}>{login}</div>
            {payoutSettingsInvalid && <ErrorWarningLine className="text-xl text-orange-500" />}
          </Menu.Button>
        </div>
        <Transition
          as={Fragment}
          enter="transition ease-out duration-100"
          enterFrom="transform opacity-0 scale-95"
          enterTo="transform opacity-100 scale-100"
          leave="transition ease-in duration-75"
          leaveFrom="transform opacity-100 scale-100"
          leaveTo="transform opacity-0 scale-95"
        >
          <Menu.Items
            onFocus={() => setMenuItemsVisible(true)}
            onBlur={() => setMenuItemsVisible(false)}
            className=" absolute right-0 mt-3 w-56 origin-top-right pt-2 pb-1
						rounded-md bg-white/5 backdrop-blur-4xl shadow-lg ring-1 ring-greyscale-50/8
						focus:outline-none z-20 overflow-hidden"
          >
            <div className="pb-2 border-b border-greyscale-50/8">
              <MenuItem secondary disabled>
                {T("navbar.profile.title").toUpperCase()}
              </MenuItem>
              <MenuItem onClick={() => setProfileSidePanelOpen(true)}>
                <User3Line className="text-xl" />
                <div className="grow">{T("navbar.profile.publicProfile")}</div>
              </MenuItem>
              <MenuItem onClick={() => setPayoutInfoSidePanelOpen(true)}>
                <MoneyDollarCircleLine className="text-xl" />
                <div className="grow">{T("navbar.profile.payoutInfo")}</div>
                {payoutSettingsInvalid && <Dot className="fill-orange-500 w-1.5" />}
              </MenuItem>
            </div>
            <MenuItem secondary onClick={logout} data-testid="logout-button">
              <LogoutBoxRLine className="text-xl" />
              {T("navbar.logout")}
            </MenuItem>
          </Menu.Items>
        </Transition>
      </Menu>
      {githubUserId && (
        <ContributorProfileSidePanel
          githubUserId={githubUserId}
          open={profileSidePanelOpen}
          setOpen={setProfileSidePanelOpen}
        />
      )}
      <PayoutInfoSidePanel
        githubUserId={githubUserId}
        open={payoutInfoSidePanelOpen}
        setOpen={setPayoutInfoSidePanelOpen}
      />
    </div>
  );
};

type MenuItemProps = {
  disabled?: boolean;
  onClick?: () => void;
  secondary?: boolean;
} & PropsWithChildren;

const MenuItem = ({ disabled = false, onClick, secondary = false, children, ...rest }: MenuItemProps) => (
  <Menu.Item
    {...rest}
    disabled={disabled}
    as="div"
    className={classNames("ui-active:bg-white/4 px-4 py-2 flex flex-row gap-3 items-center text-sm font-walsheim", {
      "cursor-pointer": !disabled,
      "cursor-default": disabled,
      "text-greyscale-50": !secondary,
      "text-spaceBlue-200": secondary,
    })}
    onClick={onClick}
  >
    {children}
  </Menu.Item>
);

export default View;
