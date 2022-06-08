<template>
  <div class="gist-content">
      <div class="Gists Table">
      <q-table
        title="Gists"
        dense
        :rows="rows"
        :columns="columns"
        row-key="id"
        @row-click="getGist"
      />
      </div>
    <pre style="text-align:left;">{{ gist }}
    </pre>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import axios from 'axios';
const rows = ref()
const gist = ref()
type rowType = {
  id: string;
  name: string;
  description: string;
  created: number;
  updated: number;
};
const columns = [
  { name: 'id', label: 'ID', align: 'left', field: 'id', sortable: true },
  { name: 'name', label: 'name', align: 'left', field: 'name', sortable: true },
  { name: 'description', label: 'Description', align: 'left', field: 'description' },
  { name: 'created', label: 'Created', align: 'left', field: 'created', sortable: true },
  { name: 'updated', label: 'Updated at', align: 'left', field: 'updated', sortable: true },
];
const getGist = async (evt, row) => {
  const res = await axios.get(`http://localhost:3000/gists/${row.id}`);
  gist.value = res.data.files[row.name].content;
};
onMounted(async () => {
  const res = await axios.get('http://localhost:3000/gists');
  rows.value = [];
  const rList = res.data as rowType[];
  const resList = rList.map((row) => {
    const mappedRow: rowType = {
      id: row.id,
      name: Object.keys(row.files)[0],
      description: row.description,
      created: row.created_at,
      updated: row.updated_at,
    };
    return mappedRow;
  });
  rows.value = resList;
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.gist-content {
  padding-left:   50px;
  padding-right:  50px;
  padding-top:    50px;
  padding-bottom: 50px;
}
</style>
