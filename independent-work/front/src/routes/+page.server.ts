import type { Actions } from "./$types";
import { SERVER_URL } from "$env/static/private";
import type { SortResults, SortStatsResult } from "./types";

export const actions = {
  sortTest: async ({ request }) => {
    const data = await request.formData();
    const qty = Number(data.get("qty"));
    const init_sort = data.get("init_sort");

    const fetchPromises = [
      fetch(`${SERVER_URL}/sort/bubble`, {
        method: "POST",
        headers: { "content-type": "text/json" },
        body: JSON.stringify({
          qty,
          init_sort
        })
      }),
      fetch(`${SERVER_URL}/sort/insertion`, {
        method: "POST",
        headers: { "content-type": "text/json" },
        body: JSON.stringify({
          qty,
          init_sort
        })
      }),
      fetch(`${SERVER_URL}/sort/selection`, {
        method: "POST",
        headers: { "content-type": "text/json" },
        body: JSON.stringify({
          qty,
          init_sort
        })
      }),
      fetch(`${SERVER_URL}/sort/merge`, {
        method: "POST",
        headers: { "content-type": "text/json" },
        body: JSON.stringify({
          qty,
          init_sort
        })
      }),
      fetch(`${SERVER_URL}/sort/quicksort`, {
        method: "POST",
        headers: { "content-type": "text/json" },
        body: JSON.stringify({
          qty,
          init_sort
        })
      })
    ];

    const results = await Promise.all(fetchPromises);
    const sortResults = {
      bubble: (await results[0].json()) as SortResults,
      insertion: (await results[1].json()) as SortStatsResult,
      selection: (await results[2].json()) as SortResults,
      merge: (await results[3].json()) as SortStatsResult,
      quicksort: (await results[4].json()) as SortResults
    };

    return sortResults;
  }
} satisfies Actions;
