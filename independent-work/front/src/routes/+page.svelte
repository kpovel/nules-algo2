<script lang="ts">
  import { enhance } from "$app/forms";
  import type { ActionData } from "./$types";
  let isSorting = false;

  export let form: ActionData;
</script>

<form
  action="?/sortTest"
  method="post"
  use:enhance={() => {
    isSorting = true;
    return async ({ update }) => {
      await update();
      isSorting = false;
    };
  }}
>
  <label>
    Select number of elements:
    <input type="number" name="nElements" required class="rounded-lg" />
  </label>
  <button
    type="submit"
    class="block rounded-lg border bg-sky-300 p-2 hover:bg-sky-400 active:bg-sky-500"
  >
    Test sort methods
  </button>
</form>

{#if isSorting}
  Sorting, please wait
{/if}

{#if form}
  <div class="py-8">
    <div class="mb-5 text-center text-xl font-semibold">
      Comparison table for {form.merge.qty} elements
    </div>
    <div
      class="grid grid-cols-6 justify-center gap-5 rounded-lg bg-white p-5 text-center border"
    >
      <div class="font-semibold">Sort method</div>
      {#each Object.entries(form) as [_, value]}
        <div>{value.method}</div>
      {/each}
      <div class="font-semibold">Sort time, sec</div>
      {#each Object.entries(form) as [_, value]}
        <div class="font-light">
          {value.sort_time.secs}.{value.sort_time.nanos}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    appearance: unset;
  }

  input[type="number"] {
    appearance: unset;
    -moz-appearance: textfield;
  }
</style>
