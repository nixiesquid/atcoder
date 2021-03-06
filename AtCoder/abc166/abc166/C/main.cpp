#include <bits/stdc++.h>
using namespace std;


void solve(long long N, long long M, std::vector<long long> H, std::vector<long long> A, std::vector<long long> B){
  unordered_map<int, bool> m;
  for (int i=0; i<N; i++) m[i] = true;

  for (int i=0; i<M; i++) {
    if (H[A[i]-1] <= H[B[i]-1]) m[A[i]-1] = false;
    if (H[B[i]-1] <= H[A[i]-1]) m[B[i]-1] = false;
  }

  int cnt = 0;
  for (int i=0; i<N; i++) if (m[i]) cnt++;
  cout << cnt << endl;
}

// Generated by 1.1.6 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
int main(){
    long long N;
    scanf("%lld",&N);
    long long M;
    scanf("%lld",&M);
    std::vector<long long> H(N);
    for(int i = 0 ; i < N ; i++){
        scanf("%lld",&H[i]);
    }
    std::vector<long long> A(M);
    std::vector<long long> B(M);
    for(int i = 0 ; i < M ; i++){
        scanf("%lld",&A[i]);
        scanf("%lld",&B[i]);
    }
    solve(N, M, std::move(H), std::move(A), std::move(B));
    return 0;
}
