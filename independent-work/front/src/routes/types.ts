export type SortResults = {
  method: string;
  qty: number;
  sort_time: {
    secs: number;
    nanos: number;
  };
};

export type SortStatsResult = {
  method: string;
  init_sort: InitSort;
  qty: number;
  sort_time: {
    secs: number;
    nanos: number;
  };
  compare: number;
  swap: number;
  memory_usage: number;
};

export enum InitSort {
  Sorted = "Sorted",
  Random = "Random",
  ReverceSorted = "ReverceSorted"
}
