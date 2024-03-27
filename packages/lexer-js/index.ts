import { parse as nativeParse  } from "./binding";

export const parse = (code: string) => {
  const buf = nativeParse(Buffer.from(code))

  return JSON.parse(buf.toString())
}
