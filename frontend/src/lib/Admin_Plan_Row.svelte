<script>

  export let id;

  let row_list = ["7b", "5-6", "D", "Mustermann", "Horst", "D", "400", "Vertretung", "-"]


</script>

<tr>

  {#each row_list as item}

    <td><input type="text" value={item} class="input-field" style="width: 90%;"></td>

  {/each}

</tr>

<style>
  .input-field {

    width: auto;
  }

  ::placeholder {

    color: black;

    opacity: 1;

  }

</style>
