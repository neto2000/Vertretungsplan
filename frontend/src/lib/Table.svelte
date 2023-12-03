<script>
  import TableRow from "./TableRow.svelte";

  let test_rows = 10
</script>


<div class="table">
  <table>
    <tr>
      <th>Klasse</th>
      <th>Stunde</th>
      <th>altes Fach</th>
      <th>abwesend</th>
      <th>Vertreter</th>
      <th>neues Fach</th>
      <th>Raum</th>
      <th>Typ</th>
      <th>Info</th>
    </tr>

    {#each Array(test_rows) as _, i}
      <TableRow /> 
    {/each}

  </table>
</div>


<style>

</style>
