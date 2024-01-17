<script>
  import AdminPlanRow from "./Admin_Plan_Row.svelte";

  let rows = 1;

</script>


<div class="table-container">
  <table class="table">
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

    {#each Array(rows) as _, i}
      <AdminPlanRow id={i} /> 
    {/each}

  </table>

  <button class="add_button" on:click={() => {rows++}}>Zeile hinzufügen</button>
</div>


<style>
  .table {
    border-collapse:separate;

    border-spacing: 0px;

    border-radius: 10px;

    border: solid;

    border-width: 1px;

    width: 100%;


    overflow: hidden;

    


    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);
    
  } 
  .table-container {

    width: 90%;


    margin-top: 15vh;

    margin-bottom: 4vh;



  }
</style>
