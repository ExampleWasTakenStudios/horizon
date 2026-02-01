export interface InflightQuery {
  originalId: number;
  horizonId: number;
  clientIP: string;
  clientPort: number;
  receivedTimestamp: number;
  timeout: NodeJS.Timeout;
}
