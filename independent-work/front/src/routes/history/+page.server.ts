import { SERVER_URL } from "$env/static/private";
import type { InitSort } from "../types";
import type { PageServerLoad } from "./$types";

type ValueMap = {
  value_map: {
    id: { value: string };
    qty: { value: string };
    init_sort: { value: InitSort };
    method: { value: string };
    sort_time: { value: string };
    compare: { value?: string };
    swap: { value?: string };
    memory_usage: { value?: string };
  };
};

export const load: PageServerLoad = async () => {
  const responce = await fetch(`${SERVER_URL}/sort/history`);
  const history = (await responce.json()) as ValueMap[];

  return {
    history
  };
};
