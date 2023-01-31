describe("As a public user, I", () => {
  const OFUX = 595505;
  const ANTHO = 990474;

  beforeEach(function () {
    cy.createGithubUser(98735558).then(user => {
      cy.createProjectWithLeader(user, "Project with budget", 1000, 493591124)
        .as("projectId")
        .then(projectId => {
          cy.requestPayment(projectId, 100, OFUX, {
            workItems: [
              "https://github.com/onlydustxyz/starkonquest/pull/68",
              "https://github.com/onlydustxyz/starkonquest/pull/68",
            ],
          })
            .asRegisteredUser(user)
            .data("requestPayment");
          cy.requestPayment(projectId, 100, OFUX, {
            workItems: ["https://github.com/onlydustxyz/starkonquest/pull/68"],
          })
            .asRegisteredUser(user)
            .data("requestPayment");
          cy.requestPayment(projectId, 500, ANTHO, {
            workItems: ["https://github.com/onlydustxyz/starkonquest/pull/68"],
          })
            .asRegisteredUser(user)
            .data("requestPayment");
        });
    });
  });

  it("can see the contributors of a project", function () {
    cy.visit(`http://localhost:5173/projects/${this.projectId}`);

    cy.contains("Contributors").click();

    cy.get("#contributors_table tbody tr:nth-child(1)").within(() => {
      cy.get("td:nth-child(1)").should("have.text", "abuisset");
      cy.get("td:nth-child(2)").should("have.text", "$500");
      cy.get("td:nth-child(3)").should("have.text", "1");
    });

    cy.get("#contributors_table tbody tr:nth-child(2)").within(() => {
      cy.get("td:nth-child(1)").should("have.text", "ofux");
      cy.get("td:nth-child(2)").should("have.text", "$200");
      cy.get("td:nth-child(3)").should("have.text", "3");
    });
  });

  it("can sort the contributors of a project", function () {
    cy.visit(`http://localhost:5173/projects/${this.projectId}`);

    cy.contains("Contributors").click();
    cy.get("#contributors_table thead tr th:nth-child(1)").click(); // sort by contributor name ASC

    cy.get("#contributors_table tbody tr:nth-child(1)").within(() => {
      cy.get("td:nth-child(1)").should("have.text", "ofux");
      cy.get("td:nth-child(2)").should("have.text", "$200");
      cy.get("td:nth-child(3)").should("have.text", "3");
    });

    cy.get("#contributors_table tbody tr:nth-child(2)").within(() => {
      cy.get("td:nth-child(1)").should("have.text", "abuisset");
      cy.get("td:nth-child(2)").should("have.text", "$500");
      cy.get("td:nth-child(3)").should("have.text", "1");
    });
  });
});
