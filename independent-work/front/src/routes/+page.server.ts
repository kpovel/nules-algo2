import type { Actions } from "./$types";
import { SERVER_URL } from "$env/static/private";

type SortResults = {
  method: string,
  qty: number,
  sort_time: {
    secs: number,
    nanos: number,
  }
}

export const actions = {
  sortTest: async ({ request }) => {
    const data = await request.formData();
    const nElements = Number(data.get("nElements"));

    const fetchPromises = [
      fetch(`${SERVER_URL}/sort/bubble/${nElements}`),
      fetch(`${SERVER_URL}/sort/insertion/${nElements}`),
      fetch(`${SERVER_URL}/sort/selection/${nElements}`),
      fetch(`${SERVER_URL}/sort/merge/${nElements}`),
      fetch(`${SERVER_URL}/sort/quicksort/${nElements}`)
    ];

    const results = await Promise.all(fetchPromises);
    const sortResults = {
      bubble: await results[0].json() as SortResults,
      insertion: await results[1].json() as SortResults,
      selection: await results[2].json() as SortResults,
      merge: await results[3].json() as SortResults,
      quicksort: await results[4].json() as SortResults,
    };

    return sortResults;
  }
} satisfies Actions;
