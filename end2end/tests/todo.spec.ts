import { test, expect, Page } from "@playwright/test";

import { $ } from "bun";

test.beforeEach(async ({ page }) => {
  await $`make reset-dev-db`;
  await page.goto("http://localhost:3000/");
});

const TODOS = ["buy milk", "clean house", "walk dog", "do homework"] as const;

test.describe("general", () => {
  test("has title", async ({ page }) => {
    await expect(page).toHaveTitle("todo app");
  });
});

test.describe("new todo", () => {
  test("can add new todo with enter", async ({ page }) => {
    const input = page.getByPlaceholder("What needs to be done?");

    await input.fill(TODOS[0]);
    await input.press("Enter");

    await expect(page.getByRole("listitem")).toHaveCount(1);
    await expect(page.getByRole("listitem")).toHaveText(TODOS[0]);
    await page.reload();
    await expect(page.getByRole("listitem")).toHaveText(TODOS[0]);
  });
  test("can add new todo with button", async ({ page }) => {
    page.getByPlaceholder("What needs to be done?").fill(TODOS[1]);

    await page.getByRole("button", { name: "Add" }).click();

    await expect(page.getByRole("listitem")).toHaveCount(1);
    await expect(page.getByRole("listitem")).toHaveText(TODOS[1]);
    await page.reload();
    await expect(page.getByRole("listitem")).toHaveText(TODOS[1]);
  });
  test("test has independent database", async ({ page }) => {
    await expect(page.getByRole("listitem")).toHaveCount(0);
  });
  test("input is reset after adding todo", async ({ page }) => {
    const input = page.getByPlaceholder("What needs to be done?");
    await input.fill(TODOS[0]);
    await input.press("Enter");
    await expect(input).toHaveText("");
  });
  test("can delete todo", async ({ page }) => {
    await createTodos(page);
    await page
      .getByRole("listitem")
      .filter({ hasText: TODOS[1] })
      .first()
      .getByLabel("delete")
      .click();
    await expect(page.getByRole("listitem")).toHaveText([
      TODOS[0],
      // TODOS[1], // deleted
      TODOS[2],
      TODOS[3],
    ]);
  });
  test("can toggle todo", async ({ page }) => {
    await createTodos(page);
    const todos = page.getByRole("listitem").getByRole("checkbox");
    await todos.nth(1).check();
    await todos.nth(3).check();

    page.reload();

    await expect(todos.nth(0)).toBeChecked({checked: false});
    await expect(todos.nth(1)).toBeChecked({checked: true});
    await expect(todos.nth(2)).toBeChecked({checked: false});
    await expect(todos.nth(3)).toBeChecked({checked: true});
  });
});

async function createTodos(page: Page) {
  const input = page.getByPlaceholder("What needs to be done?");
  for (const todo of TODOS) {
    await input.fill(todo);
    await input.press("Enter");
  }
  await expect(page.getByRole("listitem")).toHaveText(TODOS);
}
