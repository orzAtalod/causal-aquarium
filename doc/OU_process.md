# OU Process

$$
\newcommand{\d}{\mathrm{d}}
\newcommand{\E}{\mathrm{E}}
\newcommand{\var}{\mathrm{Var}}
$$

## 定义

$$P(x_{t+\d t} | x_t,\mu,\theta,\sigma,\d t) = N(x_t+\theta(\mu-x_t)， \sigma^2\d t)$$

## 前置知识

### Ito integration（伊藤积分）

一般的黎曼积分是在细化之后区间中任取一点，这一点不适用于随机过程，而伊藤积分是细化之后固定取左端点，这样就可以积分了。

$$\int_0^TX_t\ \d W_t = \lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}X_{t_i}(W_{t_{i+1}}-W_{t_i})$$

#### 期望

$$\mathrm E\left [\int_0^TX_t\ dW_t\right ] = 0$$

proove:

$$
\begin{aligned}
\mathrm E\left [\int_0^TX_t\ \d W_t\right ] &= E\left[\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}X_{t_i}(W_{t_{i+1}}-W_{t_i})\right]\\&=\lim_{n\rightarrow\infty}\sum_{k=0}^{n-1}\mathrm E\left [X_{t_i}(W_{t_{i+1}}-W_{t_i})\right ]\\&=0

\end{aligned}
$$

不需要控制收敛定理，因为是将 $\lim$ 提到 $\mathrm E$ 外边。

#### 方差（Ito isometry）

$$\mathrm E\left [\left(\int_0^TX_t\ \d W_t\right)^2\right] = \mathrm E\left [\int_0^TX_t^2\ \d t\right ]$$

proove:

$$
\begin{aligned}
\mathrm E\left [\left(\int_0^TX_t\ \d W_t\right)^2\right] &= \mathrm E\left[\left(\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}X_{t_i}(W_{t_{i+1}}-W_{t_i})\right)^2\right]\\
&=\mathrm E\left[\lim_{n\rightarrow\infty}\left(\sum_{i=0}^{n-1}X_{t_i}(W_{t_{i+1}}-W_{t_i})\right)^2\right]\\
&=\lim_{n\rightarrow\infty}\mathrm E\left[\left(\sum_{i=0}^{n-1}X_{t_i}(W_{t_{i+1}}-W_{t_i})\right)^2\right]\\
&=\lim_{n\rightarrow\infty}\mathrm E\left[\sum_{i=0}^{n-1}\sum_{j=0}^{n-1}X_{t_i}X_{t_j}(W_{t_{i+1}}-W_{t_i})(W_{t_{j+1}}-W_{j_i})\right]\\
&=\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}\sum_{j=0}^{n-1}\mathrm E\left[X_{t_i}X_{t_j}(W_{t_{i+1}}-W_{t_i})(W_{t_{j+1}}-W_{j_i})\right]\\
&=\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}\sum_{j=0}^{n-1}\mathrm E\left[X_{t_i}X_{t_j}(W_{t_{i+1}}-W_{t_i})(W_{t_{j+1}}-W_{j_i})\right]\\
&=\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}\mathrm E\left[X_{t_i}^2(W_{t_{i+1}}-W_{t_i})^2\right]\\
&=\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}\mathrm E\left[X_{t_i}^2\right ]\mathrm E\left[(W_{t_{i+1}}-W_{t_i})^2\right | X_{t_i}^2]\\
&=\lim_{n\rightarrow\infty}\sum_{i=0}^{n-1}\mathrm E\left[X_{t_i}^2\right ]\d t\\
&=\int_0^T\mathrm E\left[X_t^2\right ]\d t\\
&=\mathrm E\left [\int_0^TX_t^2\ \d t\right ]
\end{aligned}
$$

#### 链式法则（Ito引理）

微分形式：

$$
\begin{aligned}
\d X_t&=\mu(X_t,t)\d t+\sigma(X_t,t)\d W_t&\Rightarrow\\\d f(X_t,t)&=\frac{\partial f}{\partial t}\d t+\frac{\partial f}{\partial x}\d X_t+\frac 12\frac{\partial^2 f}{\partial x^2}(\sigma(X_t,t)^2)\d t
\end{aligned}
$$

积分形式：

$$
f(X_T,T) = f(X_0,0)+\int_0^T\frac{\partial f(t,X_t)}{\partial t}\d t+\int_0^T\frac{\partial f(t,X(t))}{\partial x}\d X(t)+\frac12\int_0^T\frac{\partial^2f(t,X(t))}{\partial x^2}\sigma^2(t)\d t
$$

proove: 采用微分形式

$$
\begin{aligned}
\d f(X_t,t)&=\frac{\partial f}{\partial t}\d t+\frac{\partial f}{\partial x}\d X_t+\frac12\frac{\partial^2 f}{\partial x^2}(\d X_t)^2+o(\d X_t)\\
&=\frac{\partial f}{\partial t}\d t+\frac{\partial f}{\partial x}\d X_t+\frac 12\frac{\partial^2 f}{\partial x^2}(\mu(X_t,t)\d t+\sigma(X_t,t)\d W_t)^2\\
(\mu(X_t,t)\d t+\sigma(X_t,t)\d W_t)^2 &= (\mu(X_t,t)^2)\d t^2+2\mu(X_t,t)\sigma(X_t,t)\d t\d W_t+(\sigma(X_t,t)^2)\d t\\
&=(\sigma(X_t,t)^2)\d t+o(\d t)\\
\d f(X_t,t)&=\frac{\partial f}{\partial t}\d t+\frac{\partial f}{\partial x}\d X_t+\frac12\frac{\partial^2 f}{\partial x^2}(\sigma(X_t,t)^2)\d t
\end{aligned}
$$

## 解析解

OU_process可以写成解析形式微分方程：

$$
\d X_t=\theta(\mu-X_t)\d t+\sigma\d W_t\Rightarrow\\
X_{t} = \mu+(X_0-\mu)e^{-\theta t}+\sigma\sqrt\frac{1-e^{-2\theta t}}{2\theta}N(0,1)
$$

然后对这一SDE进行求解：

$$
\begin{aligned}
\d X_t+\theta X_t\d t &= \theta\mu\d t+\sigma\d W_t\\
e^{\theta t}\d X_t+\theta e^{\theta t}X_t\d t&=\theta e^{\theta t}\mu\d t+e^{\theta t}\sigma\d W_t\\
\d(e^{\theta t}X_t)&=\theta e^{\theta t}\mu\d t+e^{\theta t}\sigma\d W_t\\
e^{\theta t}X_t-X_0&=\int_0^t \theta e^{\theta s}\mu\d s+\int_0^t e^{\theta s}\sigma\d W_s\\
&=\mu(e^{\theta t}-1)+\sigma\int_0^t e^{\theta s}\d W_s\\
X_t &=\mu+(X_0-\mu)e^{-\theta t}+\sigma e^{-\theta t}\int_0^t e^{\theta s}\d W_s
\end{aligned}
$$

注意到随机项 $\int_0^t e^{\theta s}\d W_s$ 为正态随机变量，均值为0，方差可由Ito等距性得出：

$$
\begin{aligned}
\mathrm{Var}(\int_0^t e^{\theta s}\d W_s) &= \mathrm E[\int_0^t e^{2\theta s}\d s]\\
&=\frac1{2\theta}(e^{2\theta t}-1)
\end{aligned}
$$

计算得：

$$
\begin{aligned}
\mathrm E(X_t) &= \mu+(X_0-\mu)e^{-\theta t}\\
\mathrm{Var} (X_t) &=\sigma^2e^{-2\theta t}\frac1{2\theta}(e^{2\theta t}-1)\\
&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta t})
\end{aligned}
$$

故有：

$$
X_t=N(\mu+(X_0-\mu)e^{-\theta t}, \frac{\sigma^2}{2\theta}(1-e^{-2\theta t}))
$$

## 离散简化

OU_process的离散简化过程可以将连续过程切成 $T/\Delta t$ 个不同的时间片，每个时间片执行正态模拟。每个时间片中，可以按照定义 $X_{t+\Delta t}=N(X_t+\theta(\mu-X_t)， \sigma^2\Delta t)$，也可以按照解析式 $X_{t+\Delta t}=N(\mu+(X_t-\mu)e^{-\theta\Delta t}, \frac{\sigma^2}{2\theta}(1-e^{-2\theta\Delta t}))$。以下将分析这两个方法各自的误差。

对于准确的过程，有：

$$\begin{aligned}
\mathrm E(X_T)&=\mu+(\E(X_0)-\mu)e^{-\theta T}\\
\mathrm{Var}(X_T)&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta T})
\end{aligned}$$

对于 $T=N\Delta t$ 时刻，按照定义的离散近似有：

$$\begin{aligned}
\mathrm E(X_T) &= (1-\theta)\mathrm \E(X_{T-\Delta t}) + \theta\mu\\
&=(1-\theta)((1-\theta)\mathrm \E(X_{T-2\Delta t})+\theta\mu)+\theta\mu\\
&=(1-\theta)^N\mathrm E(X_0)+\sum_{i=0}^{N-1}(1-\theta)^i\theta\mu\\
&=(1-\theta)^N\mathrm E(X_0)+(1-(1-\theta)^N)\mu\\
&=\mu+(1-\theta)^N(\E(X_0)-\mu)
\end{aligned}$$

可以发现这种离散近似就是垃圾。均值严重依赖于分片数$N$，并没有$\Delta t$来与$N$结合成合适的能近似于$e^{-\theta T}$的东西。

对于分片进行解析模拟的情况：

$$\begin{aligned}
\E(X_T)-\mu&=(\E(X_{T-\Delta t})-\mu)e^{-\theta\Delta t}\\
&=(E(X_0)-\mu)e^{-\theta T}
\end{aligned}$$

可以发现均值估计完全准确。

$$\begin{aligned}
\var(X_T)&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta\Delta t})+\var(\mu+(X_{T-\Delta t}-\mu)e^{-\theta\Delta t})\\
&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta\Delta t})+e^{-2\theta\Delta t}\var(X_{T-\Delta t})\\
&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta\Delta t})\sum_{k=0}^{N-1}e^{-2k\theta\Delta t}\\
&=\frac{\sigma^2}{2\theta}(1-e^{-2\theta T})
\end{aligned}$$

可以发现方差估计也完全准确。可以知道分片进行离散模拟是完全准确的模拟方法，前提是在每一片中采用连续时间SDE解的解析式。
