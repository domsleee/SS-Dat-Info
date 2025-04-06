
export function tryParseInt(value: string | number | undefined): number | undefined {
  if (value === undefined) return undefined;
  const parsedValue = parseInt((value as unknown as string) ?? '');
  return isNaN(parsedValue) ? undefined : parsedValue;
}