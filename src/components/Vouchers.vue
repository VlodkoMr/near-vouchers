<template>
  <main>
    <Header></Header>

    <div class="container-lg">
      <h2 class="text-center mb-5">My Payment Vouchers</h2>

      <div v-if="is_ready">
        <p v-if="!vouchers.length" class="text-center">
          <b>You don't have vouchers, please create new one:</b>
        </p>
        <div :class="{'vouchers': vouchers.length, 'no-vouchers': !vouchers.length}">
          <div v-for="voucher in vouchers" :key="voucher.id" class="one-voucher" :class="{
            'is-used': voucher.used_by,
            'is-expired': isExpired(voucher.expire_date),
          }">
            <div class="additional-border"></div>
            <div class="voucher-inner">
              <div class="canvas text-center">
                <qrcode-vue :value="getUrl(voucher.id)" :size="220" level="H"/>
              </div>
              <div class="gift-shadow"></div>
              <div class="gift-vertical">
                <img src="../assets/bn-1.svg" alt="">
              </div>
              <div class="black-bg">
                <input type="hidden" :id="'clone-'+voucher.id" readonly :value="getUrl(voucher.id)"/>
                <img src="../assets/delete.png"
                     alt="remove"
                     title="remove"
                     class="small-button remove-voucher"
                     @click="removeVoucher(voucher.id)">

                <h4 class="text-center near-amount">
                  <span :class="{'is-used': voucher.used_by}">
                    {{ toNearAmount(voucher.deposit_amount) }} NEAR
                  </span>
                  <small class="used-label" v-if="!voucher.used_by && voucher.expire_date">
                    Expire: {{ dateFormat(voucher.expire_date) }}
                  </small>

                  <small class="used-label" v-if="voucher.used_by">Used By: {{ voucher.used_by }}</small>
                  <small class="used-label" v-if="voucher.paid_amount">Paid: {{ toNearAmount(voucher.paid_amount) }} NEAR</small>
                </h4>

                <div class="copy" @click="copyURL(voucher.id)">
                  <img src="../assets/copy.png"
                       alt="copy"
                       title="copy">
                  Copy URL
                </div>
              </div>
            </div>
          </div>

          <div class="one-voucher">
            <form @submit.prevent="addVoucher" class="row">
              <div class="col-6 offset-3">
                <div class="additional-border"></div>
                <h4 class="text-center text-uppercase mb-3">
                  New Voucher
                </h4>
                <div class="mb-3">
                  <label class="form-check-label mb-1">Expire Date</label>
                  <b-form-input type="date" placeholder="Expire date" v-model="voucherExpire"></b-form-input>
                </div>
                <label class="form-check-label mb-1">NEAR Amount<sup>*</sup></label>
                <b-input-group>
                  <b-form-input type="number" required min="0.1" max="10" step="0.1" v-model="voucherDeposit"></b-form-input>
                  <b-input-group-append>
                    <b-button variant="primary" type="submit">ADD</b-button>
                  </b-input-group-append>
                </b-input-group>
              </div>
            </form>
          </div>
        </div>
      </div>
      <div v-if="!is_ready" class="text-center">
        <img src="../assets/loader.gif" alt="" width="70">
      </div>
    </div>
  </main>
</template>

<script>
import Big from "big.js";
import sha256 from 'crypto-js/sha256';
import QrcodeVue from 'qrcode.vue';
import Header from "./Header";

export default {
  name: "Vouchers",
  beforeMount() {
    if (this.isSignedIn) {
      if (!localStorage.getItem('app-private-keys')) {
        localStorage.setItem('app-private-keys', JSON.stringify({}));
      }

      this.getVouchers();
    }
  },
  components: {
    QrcodeVue,
    Header,
  },
  data: function () {
    return {
      is_ready: false,
      voucherDeposit: null,
      voucherExpire: null,
      vouchers: [],
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
  },
  methods: {
    getVouchers() {
      this.is_ready = false;
      window.contract.user_vouchers({
        'account_id': window.accountId
      }).then(vouchers => {
        this.vouchers = [];
        vouchers.forEach(voucher => {
          if (this.isValidVoucher(voucher.id)) {
            this.vouchers.push(voucher);
          }
        });
        this.is_ready = true;
      })
    },

    async addVoucher() {
      if (this.voucherDeposit > 0) {
        let expire_date = null;
        if (this.voucherExpire) {
          // convert date to blockchain timestamp
          expire_date = Date.parse(this.voucherExpire) * 1000000;
        }
        let keys = JSON.parse(localStorage.getItem('app-private-keys'));
        const newId = this.randomStr(12);
        keys[newId] = this.randomStr(64);
        localStorage.setItem('app-private-keys', JSON.stringify(keys));

        const hash = sha256(keys[newId]).toString();
        const GAS = Big(300).times(10 ** 12).toFixed();
        const DEPOSIT = Big(this.voucherDeposit).times(10 ** 24).toFixed();
        try {
          await window.contract.add_voucher({
            id: newId,
            hash,
            expire_date
          }, GAS, DEPOSIT);
        } catch (e) {
          alert("Something went wrong!");
          throw e //re-throw
        } finally {
          console.log('added');
          this.getVouchers();
        }
      } else {
        alert("Please specify deposit");
      }
    },

    async removeVoucher(id) {
      if (confirm("Please confirm voucher removing. The balance will be refunded to your wallet.")) {
        try {
          await window.contract.remove_voucher({id});
        } catch (e) {
          alert("Something went wrong!");
          throw e //re-throw
        } finally {
          console.log('removed');
          this.getVouchers();
        }
      }
    },

    getUrl(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return window.location.origin + `/?user=${window.accountId}&id=${id}#${keys[id]}`;
    },

    isValidVoucher(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return keys[id] !== undefined;
    },

    toNearAmount(value) {
      return Big(value).div(10 ** 24).toFixed();
    },

    copyURL(id) {
      const link = document.querySelector(`#clone-${id}`);
      link.setAttribute('type', 'text');
      link.select();
      document.execCommand("copy");
      link.setAttribute('type', 'hidden');
    },

    randomStr(length) {
      let result = '';
      const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
      const charactersLength = characters.length;
      for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
      }
      return result;
    },

    dateFormat(timestamp) {
      const date = new Date(timestamp / 1000000);
      return new Intl.DateTimeFormat().format(date);
    },

    isExpired(timestamp) {
      if (timestamp) {
        const date = new Date(timestamp / 1000000);
        return new Date() > date;
      }
    },
  },
}
</script>
