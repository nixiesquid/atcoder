#include <bits/stdc++.h>
using namespace std;


void solve(long long N, long long Q, std::vector<long long> t, std::vector<long long> x, std::vector<long long> y){

}

// Generated by 1.1.7.1 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
int main(){
    long long N;
    scanf("%lld",&N);
    long long Q;
    scanf("%lld",&Q);
    std::vector<long long> t(Q);
    std::vector<long long> x(Q);
    std::vector<long long> y(Q);
    for(int i = 0 ; i < Q ; i++){
        scanf("%lld",&t[i]);
        scanf("%lld",&x[i]);
        scanf("%lld",&y[i]);
    }
    solve(N, Q, std::move(t), std::move(x), std::move(y));
    return 0;
}
