<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Qty</th>
      <th>Initial Sort</th>
      <th>Method</th>
      <th>Sort Time</th>
      <th>Compare</th>
      <th>Swap</th>
      <th>Memory Usage</th>
    </tr>
  </thead>
  <tbody>
    {#each data.history as record}
      <tr>
        <td>{record.value_map.id.value}</td>
        <td>{record.value_map.qty.value}</td>
        <td>{record.value_map.init_sort.value}</td>
        <td>{record.value_map.method.value}</td>
        <td>{record.value_map.sort_time.value}</td>
        <td>{record.value_map.compare?.value || "-"}</td>
        <td>{record.value_map.swap?.value || "-"}</td>
        <td>{record.value_map.memory_usage?.value || "-"}</td>
      </tr>
    {/each}
  </tbody>
</table>
