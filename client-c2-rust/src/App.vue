<template>
    <div class="header">
        <Card style="text-align: center; height: 5rem; overflow:hidden">
            <template #header>
                <h1>Rust C2</h1>
            </template>
        </Card>
    </div>
    <div class="panel-menu"> 
        <Card style="align: center; float: right; width: 80%; height: 100%">
            <template #content>
                <p>Content</p>
            </template>
        </Card>
        <PanelMenu id="main-menu" :model="items" style="float: left; width: 20%" />
    </div>
</template>

<script setup>
import { ref } from 'vue';

const items = ref([
    {
        label: 'Listener Menu',
        icon: 'pi pi-fw pi-phone',
        items: [
            {
                label: 'New',
                icon: 'pi pi-fw pi-plus'
            },
            {
                label: 'Delete',
                icon: 'pi pi-fw pi-trash'
            },
            {
                label: 'Refresh',
                icon: 'pi pi-fw pi-refresh'
            },
        ]
    },
    {
        label: 'Agent Menu',
        icon: 'pi pi-fw pi-user',
        items: [
            {
                label: 'Save',
                icon: 'pi pi-fw pi-save'
            },
            {
                label: 'Update',
                icon: 'pi pi-fw pi-refresh'
            },
            {
                label: 'Delete',
                icon: 'pi pi-fw pi-trash'
            },
        ]
    },
    {
        label: 'Command Menu',
        icon: 'pi pi-fw pi-send',
        items: [
            {
                label: 'Print',
                icon: 'pi pi-fw pi-print'
            }
        ]
    }
]);
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.header {
    background-color: #cb997e;
    color: #fff;
    padding: 1rem;
    width: 100%;
}

.main-menu {
    background-color: #333;
    color: #fff;
    padding: 1rem;
    width: 20%;
}

.content {
    background-color: #fff;
    color: #333;
    padding: 1rem;
    width: 80%;
}

</style>
