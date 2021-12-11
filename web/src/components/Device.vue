<template>
  <div class="row" v-for="device in devices" :key="device.id">
    <div class="col s12 m6 l4 xl3">
      <div class="card">
        <div class="card-content">
          <span class="card-title">
            {{ device.name }}
          </span>
          <p>
            {{ device.mac }}
          </p>
        </div>
        <div class="card-action">
          <a @click="awake(device.name)">Awake</a>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Device } from "@/types/device.model";
import { Options, Vue } from 'vue-class-component';

@Options({
  created() {
    fetch("http://localhost:8001/wake-up/api/devices")
        .then(response => response.json())
        .then(devices => this.devices = devices);
  },
  methods: {
    awake(name: string): void {
      fetch(`http://localhost:8001/wake-up/api/awake/${name}`)
          .then(response => response.json())
          .then(console.log);
    }
  }
})
export default class DeviceList extends Vue {
  devices: Device[] = [];
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
