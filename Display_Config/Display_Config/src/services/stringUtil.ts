
export function tryParseInt(value: string | number | undefined): number | null {
  if (value === undefined) return null;
  const parsedValue = parseInt((value as unknown as string) ?? '');
  return isNaN(parsedValue) ? null : parsedValue;
}