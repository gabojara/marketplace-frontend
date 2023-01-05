describe("As a project lead, I", () => {
  beforeEach(function () {
    cy.createGithubUser(98735558)
      .then(user => {
        cy.createProjectWithLeader(user, "Project with budget", 1000, 493591124)
          .as("projectId");
        cy.wait(700);
        cy.signinUser(user)
          .then(user => JSON.stringify(user.session))
          .as("token");
      });
  });

  it("can request a payment", function () {
    cy.visit(`http://localhost:5173/projects/${this.projectId}`, {
      onBeforeLoad(win) {
        win.localStorage.setItem("hasura_token", this.token);
      },
    });

    cy.contains("Payments").click();
    cy.get("#remainingBudget").should("have.text", "$1000");

    cy.contains("Submit payment").click();
    cy.get("[name=contributor").type("AnthonyBuisset");
    cy.get("[name=linkToIssue").type("https://github.com/onlydustxyz/starkonquest/pull/68");
    cy.wait(1000);
    cy.contains("Confirm payment").click();

    cy.reload();
    cy.contains("Payments").click();
    cy.get("#remainingBudget").should("have.text", "$0");
  });
});
