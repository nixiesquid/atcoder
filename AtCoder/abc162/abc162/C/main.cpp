#include <bits/stdc++.h>
using namespace std;
using ll = long long;


ll gcd(ll a, ll b) {
  if (b == 0) return a;
  return gcd(b, a % b);
}

ll three_gcd(ll a, ll b, ll c) {
  return gcd(gcd(a, b), c);
}

void solve(ll K){
  ll sum = 0;
  for (int i=1; i<=K; i++) {
    for (int j=1; j<=K; j++) {
      for (int k=1; k<=K; k++) {
        sum += three_gcd(i, j, k);
      }
    }
  }
  cout << sum << endl;
}

// Generated by 1.1.6 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
int main(){
    ll K;
    scanf("%lld",&K);
    solve(K);
    return 0;
}
