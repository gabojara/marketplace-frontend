import { expect, Page } from "@playwright/test";
import { Project } from "../types";

export class BrowseProjectsPage {
  readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  async goto() {
    await this.page.goto("/");
  }

  async expectProjectToBeVisible(
    project: Project,
    checks?: { logoSrc?: string; contributors?: string; repositories?: string }
  ) {
    const projectLocator = this.getProjectLocator(project);
    await expect(projectLocator.getByText(project.name, { exact: true })).toBeVisible();

    if (checks?.logoSrc) {
      await expect(projectLocator.locator("img[alt='Project Logo']")).toHaveAttribute("src", checks?.logoSrc);
    }
    if (checks?.contributors) {
      await expect(projectLocator.getByText(checks?.contributors, { exact: true })).toBeVisible();
    }
    if (checks?.repositories) {
      await expect(projectLocator.getByText(checks?.repositories, { exact: true })).toBeVisible();
    }
    return projectLocator;
  }

  projects() {
    return this.page.getByTestId("project-card");
  }

  async expectProjectsToBeVisible(...projects: Project[]) {
    for (const project of projects) {
      await this.expectProjectToBeVisible(project);
    }
  }

  async expectProjectsNotToBeVisible(...projects: Project[]) {
    for (const project of projects) {
      const projectLocator = this.getProjectLocator(project);
      await expect(projectLocator.getByText(project.name, { exact: true })).not.toBeVisible();
    }
  }

  async expectFiltersToBeVisible(...filters: string[]) {
    for (const filter of filters) {
      await expect(this.getFilterLocator(filter)).toBeVisible();
    }
  }

  async expectFiltersNotToBeVisible(...filters: string[]) {
    for (const filter of filters) {
      await expect(this.getFilterLocator(filter)).not.toBeVisible();
    }
  }

  async filterBy(filter: string) {
    await this.getFilterLocator(filter).click();
  }

  async sortBy(criterion: string) {
    await this.getSortByLocator().click();
    await this.page.getByRole("option", { name: criterion }).click();
  }

  async clearFilters() {
    await this.page.getByText("Clear all", { exact: true }).click();
  }

  getProjectLocator(project: Project) {
    return this.page.locator(`a[href='/projects/${project.id}']`);
  }

  getFilterLocator(filter: string) {
    return this.page.getByText(filter, { exact: true });
  }

  getSortByLocator() {
    return this.page.getByText("Sort by");
  }
}
