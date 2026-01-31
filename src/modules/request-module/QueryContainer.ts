export class QueryContainer {
  private readonly query: Buffer;
  private readonly sourceAddress: string;
  private readonly sourcePort: number;

  public constructor(query: Buffer, sourceAddress: string, sourcePort: number) {
    this.query = query;
    this.sourceAddress = sourceAddress;
    this.sourcePort = sourcePort;
  }

  public getQueryId(): number {
    return this.query.readUint16BE(0);
  }

  public getQuery(): Buffer {
    return this.query;
  }

  public getSourceAddress(): string {
    return this.sourceAddress;
  }

  public getSourcePort(): number {
    return this.sourcePort;
  }
}
