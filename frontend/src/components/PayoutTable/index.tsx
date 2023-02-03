import { useIntl } from "src/hooks/useIntl";
import { Currency, Payment, PaymentStatus } from "src/types";
import Table from "../Table";
import Line from "../Table/Line";
import Cell from "../Table/Cell";
import Headers from "../Table/HeaderLine";
import HeaderCell, { HeaderCellWidth } from "../Table/HeaderCell";
import onlyDustLogo from "assets/img/onlydust-logo.png";
import RoundedImage from "src/components/RoundedImage";
import SortingArrow from "../ContributorsTable/SortingArrow";
import PayoutStatus from "../PayoutStatus";
import GithubPRLink, { LinkColor } from "./GithubPRLink";
import MoneyDollarCircleLine from "src/icons/MoneyDollarCircleLine";
import FocusLine from "src/icons/FocusLine";
import FolderLine from "src/icons/FolderLine";
import TimeLine from "src/icons/TimeLine";
import { formatMoneyAmount } from "src/utils/money";
import displayRelativeDate from "src/utils/displayRelativeDate";
import usePaymentSorting, { Field, Sorting } from "src/hooks/usePaymentSorting";

type PropsType = {
  payments: Payment[];
  payoutInfoMissing: boolean;
};

const PayoutTable: React.FC<PropsType> = ({ payments, payoutInfoMissing }) => {
  const { sortedPayments, sorting, applySorting } = usePaymentSorting(payments);

  return (
    <Table id="payment_table" headers={renderHeaders(sorting, applySorting)}>
      {renderPayments(sortedPayments, payoutInfoMissing)}
    </Table>
  );
};

const renderHeaders = (sorting: Sorting, applySorting: (field: Field) => void) => {
  const { T } = useIntl();
  return (
    <Headers>
      <HeaderCell width={HeaderCellWidth.Sixth} onClick={() => applySorting(Field.Date)} horizontalMargin>
        <TimeLine className="p-px font-normal" />
        <span>{T("payment.table.date")}</span>
        <SortingArrow direction={sorting.ascending ? "up" : "down"} visible={sorting.field === Field.Date} />
      </HeaderCell>
      <HeaderCell width={HeaderCellWidth.Half} onClick={() => applySorting(Field.Contribution)} horizontalMargin>
        <FolderLine className="p-px font-normal" />
        <span>{T("payment.table.contribution")}</span>
        <SortingArrow direction={sorting.ascending ? "up" : "down"} visible={sorting.field === Field.Contribution} />
      </HeaderCell>
      <HeaderCell width={HeaderCellWidth.Sixth} onClick={() => applySorting(Field.Amount)} horizontalMargin>
        <MoneyDollarCircleLine className="p-px font-normal" />
        <span>{T("payment.table.amount")}</span>
        <SortingArrow direction={sorting.ascending ? "up" : "down"} visible={sorting.field === Field.Amount} />
      </HeaderCell>
      <HeaderCell width={HeaderCellWidth.Quarter} onClick={() => applySorting(Field.Status)} horizontalMargin>
        <FocusLine className="p-px font-normal" />
        <span>{T("payment.table.status")}</span>
        <SortingArrow direction={sorting.ascending ? "up" : "down"} visible={sorting.field === Field.Status} />
      </HeaderCell>
    </Headers>
  );
};

const renderPayments = (payments: Payment[], payoutInfoMissing: boolean) => {
  return payments.map(payment => (
    <Line key={payment.id} highlightOnHover={200}>
      <Cell>{displayRelativeDate(payment.requestedAt)}</Cell>
      <Cell className="flex flex-row gap-3">
        <RoundedImage src={payment?.project?.logoUrl || onlyDustLogo} alt={payment?.project?.title || ""} />
        <div className="flex flex-col truncate justify-center">
          <div className="font-normal text-base font-belwe">{payment?.project?.title}</div>
          {payment.reason && <GithubPRLink link={payment.reason} linkColor={LinkColor.Grey}></GithubPRLink>}
        </div>
      </Cell>
      <Cell>{formatMoneyAmount(payment.amount.value, payment.amount.currency)}</Cell>
      <Cell>
        <PayoutStatus {...{ status: payment.status, payoutInfoMissing }} />
      </Cell>
    </Line>
  ));
};

// TODO: replace this any with GraphQL-generated ts types
export const mapApiPaymentsToProps = (apiPayment: any): Payment => {
  const amount = { value: apiPayment.amountInUsd, currency: Currency.USD };
  const project = apiPayment.budget.project;
  const reason = apiPayment.reason?.work_items?.at(0);
  const requestedAt = apiPayment.requestedAt;
  const getPaidAmount = (payments: { amount: number }[]) =>
    payments.reduce((total: number, payment: { amount: number }) => total + payment.amount, 0);

  return {
    id: apiPayment.id,
    requestedAt: requestedAt,
    amount,
    reason,
    project: {
      id: project.id,
      title: project.name,
      description: project.projectDetails.description,
      logoUrl: project.projectDetails.logoUrl || project.githubRepo?.content?.logoUrl,
    },
    status:
      getPaidAmount(apiPayment.payments) === apiPayment.amountInUsd
        ? PaymentStatus.ACCEPTED
        : PaymentStatus.WAITING_PAYMENT,
  };
};

export default PayoutTable;
