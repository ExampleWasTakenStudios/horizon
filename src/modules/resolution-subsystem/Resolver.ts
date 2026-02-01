export interface Resolver {
  resolveQuery(query: Buffer): void;
}
