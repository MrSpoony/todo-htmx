import { test, expect, Page } from "@playwright/test";

import { $ } from "bun";

async function resetDatabase() {
  await $`make reset-dev-db`;
}

test.beforeEach(async ({ page }) => {
  await resetDatabase();
  await page.goto("http://localhost:3000/");
});

test.afterAll(async () => {
  await resetDatabase();
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
  test("input is reset after adding todo", async ({ page }) => {
    const input = page.getByPlaceholder("What needs to be done?");
    await input.fill(TODOS[0]);
    await input.press("Enter");
    await expect(input).toHaveText("");
  });
  test("can delete todo", async ({ page }) => {
    await createTodos(page);
    await page.getByRole("listitem").nth(1).first().getByRole("button").click();
    await expect(page.getByRole("listitem")).toHaveText([
      TODOS[0],
      // TODOS[1], // deleted
      TODOS[2],
      TODOS[3],
    ]);
  });
  test("can toggle todo", async ({ page }) => {
    await createTodos(page);
    let todos = page.getByRole("listitem").getByRole("checkbox");
    await todos.nth(1).check();
    await todos.nth(3).check();

    await expect(todos.nth(0)).toBeChecked({ checked: false });
    await expect(todos.nth(1)).toBeChecked({ checked: true });
    await expect(todos.nth(2)).toBeChecked({ checked: false });
    await expect(todos.nth(3)).toBeChecked({ checked: true });

    await page.reload();

    todos = page.getByRole("listitem").getByRole("checkbox"); // re-fetch

    await expect(todos.nth(0)).toBeChecked({ checked: false });
    await expect(todos.nth(1)).toBeChecked({ checked: true });
    await expect(todos.nth(2)).toBeChecked({ checked: false });
    await expect(todos.nth(3)).toBeChecked({ checked: true });
  });

  test("empty text returns warning alert", async ({ page }) => {
    await waitForRequest(page, async () => {
      await page.getByRole("button", { name: "Add" }).click();
    });
    await expect(page.getByRole("alert")).toContainText(
      "Please enter a non-empty todo",
    );
  });

  test("warning alert disappears after 5 seconds", async ({ page }) => {
    await waitForRequest(page, async () => {
      await page.getByRole("button", { name: "Add" }).click();
    });
    await expect(page.getByRole("alert")).toHaveCount(1);
    const t = new Date();
    await waitForRequest(page)
    await expect(page.getByRole("alert")).toHaveCount(0);
    const t2 = new Date();
    const timeTaken = t2.getTime() - t.getTime();
    const diff = 5000 - timeTaken;
    expect(diff).toBeLessThanOrEqual(1000);
  });
});

async function createTodos(page: Page) {
  const input = page.getByPlaceholder("What needs to be done?");
  for (const todo of TODOS) {
    await input.click();
    await input.fill(todo);
    await waitForSettle(page, async () => {
      await page.getByRole("button", { name: "Add" }).click();
    });
  }
  await expect(page.getByRole("listitem")).toHaveText(TODOS);
}

async function waitForSettle(page: Page, fn?: () => Promise<void>) {
  return await waitFor(page, "htmx:afterSettle", fn);
}

async function waitForRequest(page: Page, fn?: () => Promise<void>) {
  return await waitFor(page, "htmx:afterRequest", fn);
}

async function waitFor(
  page: Page,
  eventName: string,
  fn?: () => Promise<void>,
) {
  const eventListener = page.evaluate(
    (eventName: string) =>
      new Promise<void>((resolve) => {
        document.addEventListener(
          eventName,
          () => {
            return resolve();
          },
          { once: true },
        );
      }),
    eventName,
  );
  if (fn) await fn();
  await eventListener;
}
