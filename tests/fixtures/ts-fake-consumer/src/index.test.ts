import { describe, expect, it } from "vitest";
import { ping } from "./index.js";

describe("ping", () => {
  it("returns 'ok'", () => {
    expect(ping()).toBe("ok");
  });
});
