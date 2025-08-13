// Reexport the native module. On web, it will be resolved to RustModule.web.ts
// and on native platforms to RustModule.ts
export { default } from "./src/RustModule";
export * from "./src/RustModule.types";

import RustModule from "./src/RustModule";

export async function rustAdd(a: number, b: number): Promise<number> {
  return await RustModule.rustAdd(a, b);
}

export function hello(): string {
  return RustModule.hello();
}
