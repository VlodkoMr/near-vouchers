<template>
  <main>
    <Header></Header>

    <div class="container" v-if="isReady">
      <h1 class="text-center">Payment</h1>

      <b-row class="text-center mt-3" v-if="!isPaymentLoader && !paymentSuccess">
        <b-col lg="4" class="m-auto">
          <p><b>Take payment from Voucher</b></p>
          <div class="one-voucher pay-voucher">
            <form @submit.prevent="transfer" class="row">
              <div class="offset-1 col-10">
                <div class="additional-border"></div>

                <div style="font-size: 20px;">
                  Balance: <b>{{ toNearAmount(voucher.deposit_amount) }} NEAR</b>
                </div>

                <div v-if="voucher.payment_type === 'static'">
                  <div v-if="voucher.paid_amount > 0">
                    <b class="text-danger">Voucher already claimed.</b>
                  </div>
                  <span v-if="voucher.paid_amount === 0 && voucher.expire_date">
                    Expire Date: {{ dateFormat(voucher.expire_date) }}
                  </span>
                  <div class="mt-3">
                    <b-button variant="primary" type="submit" class="fw-bold text-uppercase" :disabled="isExpired(voucher.expire_date)">Claim</b-button>
                  </div>
                </div>

                <div v-if="voucher.payment_type === 'linear'">
                  <span v-if="voucher.expire_date">
                    Full unlock Date: {{ dateFormat(voucher.expire_date) }}
                  </span>
                  <hr>
                  <div>
                    Claimed: {{ toNearAmount(voucher.paid_amount) }} NEAR
                  </div>
                  <div>
                    You can Claim: {{ linearClaimAmount() }} NEAR
                  </div>
                  <div class="mt-3">
                    <b-button variant="primary" type="submit" class="fw-bold text-uppercase" :disabled="linearClaimAmount() <=0">Claim</b-button>
                  </div>
                </div>

              </div>
            </form>
          </div>
        </b-col>
      </b-row>

      <b-row class="text-center mt-3 success-payment" v-if="!isPaymentLoader && paymentSuccess">
        <b-col></b-col>
        <b-col lg="5" md="6">
          <img src="../assets/success.png" alt="success" width="60">
          <p class="mt-3">
            <b>You received {{ toNearAmount(paymentAmount) }} NEAR.</b>
          </p>
        </b-col>
        <b-col></b-col>
      </b-row>

      <div v-if="isPaymentLoader" class="text-center">
        <p>Processing payment...</p>
        <img src="../assets/loader.gif" alt="" width="70">
      </div>
    </div>
  </main>
</template>

<script>
import Header from './Header';
import { toNearAmount as toNearAmountUtil, dateFormat as dateFormatUtil, isExpired as isExpiredUtil } from '../utils';

export default {
  name: "VoucherPayment",
  beforeMount() {
    if (this.isSignedIn) {
      this.key = window.location.hash.replace('#', '');
      const urlParams = new URLSearchParams(window.location.search);
      this.id = urlParams.get("id");
      this.userId = urlParams.get("user");
      this.voucherInfo();
    }
  },
  components: {
    Header,
  },
  data: function () {
    return {
      isReady: false,
      paymentAmount: null,
      paymentSuccess: false,
      isPaymentLoader: false,
      key: "",
      id: "",
      userId: "",
      voucher: {},
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
  },
  methods: {
    async voucherInfo() {
      this.isReady = false;
      try {
        this.voucher = await window.contract.voucher_info({
          id: this.id,
          account_id: this.userId,
        });
        this.isReady = true;
        console.log('voucher', this.voucher);
      } catch (e) {
        console.log('err')
      }
    },
    toNearAmount(amount) {
      return toNearAmountUtil(amount);
    },

    dateFormat(date) {
      return dateFormatUtil(date);
    },

    isExpired(timestamp) {
      return isExpiredUtil(timestamp) || this.voucher.paid_amount > 0;
    },

    linearClaimAmount() {
      return 0;
    },

    async transfer() {
      this.isPaymentLoader = true;
      let isError = false;
      try {
        this.paymentAmount = await window.contract.transfer({
          key: this.key,
          id: this.id,
          account_id: this.userId,
        });
      } catch (e) {
        isError = true;
        let errorText = 'Error: Something went wrong!';
        if (e.kind.ExecutionError) {
          const reg = /'(.*)'/g;
          let error = e.kind.ExecutionError.match(reg);
          if (error.length) {
            errorText = "Error: " + error[0].replaceAll("'", "");
          }
        }
        alert(errorText);
        throw e //re-throw
      } finally {
        this.isPaymentLoader = false;

        if (!isError) {
          this.paymentSuccess = true;
        }
      }
    },
  },
}
</script>
