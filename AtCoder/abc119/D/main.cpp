#include <bits/stdc++.h>
using namespace std;


void solve(long long A, long long B, long long Q, std::vector<long long> s, std::vector<long long> t, std::vector<long long> x){

}

// Generated by 1.1.6 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
int main(){
    long long A;
    scanf("%lld",&A);
    long long B;
    scanf("%lld",&B);
    long long Q;
    scanf("%lld",&Q);
    std::vector<long long> s(A);
    for(int i = 0 ; i < A ; i++){
        scanf("%lld",&s[i]);
    }
    std::vector<long long> t(B);
    for(int i = 0 ; i < B ; i++){
        scanf("%lld",&t[i]);
    }
    std::vector<long long> x(Q);
    for(int i = 0 ; i < Q ; i++){
        scanf("%lld",&x[i]);
    }
    solve(A, B, Q, std::move(s), std::move(t), std::move(x));
    return 0;
}