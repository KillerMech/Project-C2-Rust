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
                <p v-if="!currentComponent">Welcome to Rust C2 <br>
                    Please choose an option from the menu
                </p>
                <component :is="currentComponent" />
            </template>
        </Card>
        <PanelMenu id="main-menu" :model="items" v-model="selectedItem" style="float: left; width: 20%" />
    </div>
</template>

<script setup>
import { ref } from 'vue';
import NewListenerForm from './components/NewListenerForm.vue';
import CreateAgentForm from './components/CreateAgentForm.vue';
import RestoreListenerForm from './components/RestoreListenerForm.vue';

const currentComponent = ref(null);
const selectedItem = ref(null);

const items = ref([
    {
        label: 'Listener Menu',
        icon: 'pi pi-fw pi-phone',
        items: [
            {
                label: 'New',
                icon: 'pi pi-fw pi-plus',
                command: () => {
                    currentComponent.value = NewListenerForm;
                    selectedItem.value = 'New';
                }
            },
            {
                label: 'Delete',
                icon: 'pi pi-fw pi-trash'
            },
            {
                label: 'Restore',
                icon: 'pi pi-fw pi-undo',
                command: () => {
                    currentComponent.value = RestoreListenerForm;
                    selectedItem.value = 'Restore';
                }
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
                label: 'Create',
                icon: 'pi pi-fw pi-save',
                command: () => {
                    currentComponent.value = CreateAgentForm;
                    selectedItem.value = 'Create';
                }
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

#content {
    background-color: #fff;
    color: #333;
    padding: 1rem;
    width: 80%;
}

</style>
