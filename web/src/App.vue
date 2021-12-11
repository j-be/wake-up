<template>
  <div class="container">
    <DeviceList></DeviceList>
  </div>
</template>

<script lang="ts">
import {Options, Vue} from 'vue-class-component';
import DeviceList from '@/components/Device.vue';

@Options({
  components: {
    DeviceList,
  },
  methods: {
    methods: {
      awake(name: string): void {
        console.log(name);
      }
    },
  }
})
export default class App extends Vue {}
</script>
<style lang="sass">

$card-bg-color: #1e1e1e
$off-black: #fff

@import '../node_modules/materialize-css/sass/materialize'

html
  background-color: #121212

</style>
