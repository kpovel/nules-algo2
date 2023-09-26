import type {Actions} from "./$types";
import {SERVER_URL} from "$env/static/private";
import type {SortStatsResult} from "../types";

export const actions = {
  sortTest: async ({ request }) => {
    const data = await request.formData();
    const qty = Number(data.get("qty"));
    const init_sort = data.get("init_sort");

    const fetchPromises = [
      fetch(`${SERVER_URL}/sort/insertion`, {
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
      })
    ];

    const results = await Promise.all(fetchPromises);
    return {
      insertion: (await results[0].json()) as SortStatsResult,
      merge: (await results[1].json()) as SortStatsResult
    };
  }
} satisfies Actions;
