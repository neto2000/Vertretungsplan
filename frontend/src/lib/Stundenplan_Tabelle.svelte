<script>
  
  export let is_edit = false;

  let table = [
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
  ];

</script>


<div>
  <tr>
    <th></th>
    <th>Montag</th>
    <th>Dienstag</th>
    <th>Mittwoch</th>
    <th>Donnerstag</th>
    <th>Freitag</th>
  </tr>
  {#each table as row, i (i)}
    <tr>
    <td>{i + 1}. Stunde</td>
    {#each row as j}
        <td>
          {j.fach}

        </td>
    {/each}

    </tr>
  {/each} 
</div>


<style>

</style>
