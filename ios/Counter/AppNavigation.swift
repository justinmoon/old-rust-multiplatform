import SwiftUI

struct AppNavigation: View {
    @State var rust: ViewModel
    @State private var navigationPath = NavigationPath()
    @State private var showSuccessScreen = false
    @State private var showErrorScreen = false
    @State private var successMessage = "Transaction completed successfully!"
    @State private var errorMessage = "Transaction failed. Please try again."
    @State private var screenStack: [AppScreen] = []

    // Public initializer
    init(rust: ViewModel) {
        self._rust = State(initialValue: rust)
    }

    var body: some View {
        TabView(selection: tabSelection) {
            NavigationStack(path: $navigationPath) {
                HomeView(rust: rust)
                    .navigationDestination(for: AppScreen.self) { screen in
                        switch screen {
                        case .mint:
                            MintView(rust: rust)
                        case .mintAmount:
                            MintAmountView(rust: rust)
                        case .mintConfirm:
                            MintConfirmView(rust: rust)
                        case .melt:
                            MeltView(rust: rust)
                        case .meltConfirm:
                            MeltConfirmView(rust: rust)
                        default:
                            EmptyView()
                        }
                    }
            }
            .tabItem {
                Label("Home", systemImage: "house")
            }
            .tag(0)

            TransactionHistoryView(rust: rust)
                .tabItem {
                    Label("History", systemImage: "clock")
                }
                .tag(1)
        }
        .onChange(of: rust.router.route) { _, newRoute in
            handleRouteChange(newRoute)
        }
        .fullScreenCover(isPresented: $showSuccessScreen) {
            SuccessView(rust: rust, message: successMessage) {
                // Reset to home on dismiss
                rust.dispatch(event: .setRoute(route: .home))
                showSuccessScreen = false
            }
        }
        .fullScreenCover(isPresented: $showErrorScreen) {
            ErrorView(
                rust: rust,
                error: errorMessage,
                onRetry: {
                    showErrorScreen = false
                },
                onQuit: {
                    rust.dispatch(event: .setRoute(route: .home))
                    showErrorScreen = false
                }
            )
        }
    }

    // Helper binding for tab selection that syncs with router
    private var tabSelection: Binding<Int> {
        Binding<Int>(
            get: {
                switch rust.router.route {
                case .transactionHistory: return 1
                default: return 0
                }
            },
            set: { newValue in
                if newValue == 1 {
                    rust.dispatch(event: .setRoute(route: .transactionHistory))
                } else {
                    rust.dispatch(event: .setRoute(route: .home))
                }
            }
        )
    }

    private func handleRouteChange(_ route: Route) {
        switch route {
        case .home:
            // Clear navigation stack when returning to home
            if navigationPath.count > 0 {
                navigationPath.removeLast(navigationPath.count)
            }
        case .mint:
            updateNavigationPath(to: .mint)
        case .mintAmount:
            updateNavigationPath(to: .mintAmount)
        case .mintConfirm:
            updateNavigationPath(to: .mintConfirm)
        case .melt:
            updateNavigationPath(to: .melt)
        case .meltConfirm:
            updateNavigationPath(to: .meltConfirm)
        case .success:
            showSuccessScreen = true
        case .error:
            showErrorScreen = true
        default:
            break
        }
    }

    private func updateNavigationPath(to screen: AppScreen) {
        // Check if the last screen is the same as the one we want to navigate to
        if let lastScreen = screenStack.last, lastScreen == screen {
            return
        }

        // Update the screen stack and navigation path
        screenStack.append(screen)
        navigationPath.append(screen)
    }
}

// Helper enum to use with NavigationStack
enum AppScreen: Hashable {
    case home
    case mint
    case mintAmount
    case mintConfirm
    case melt
    case meltConfirm
    case transactionHistory
}

// MARK: - View Implementations

struct HomeView: View {
    @State var rust: ViewModel

    var body: some View {
        VStack {
            Text("21,000 sats")
                .font(.largeTitle)
                .padding()

            HStack(spacing: 20) {
                Button(action: {
                    rust.dispatch(event: .setRoute(route: .mint))
                }) {
                    Text("Mint")
                        .frame(width: 100, height: 50)
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: {
                    rust.dispatch(event: .setRoute(route: .melt))
                }) {
                    Text("Melt")
                        .frame(width: 100, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }

            // Add animation for jiggling buttons when transaction exists
            // as shown in your mockup
            .padding()
        }
        .navigationTitle("Home")
    }
}

struct TransactionHistoryView: View {
    @State var rust: ViewModel

    var body: some View {
        List {
            ForEach(0..<3) { _ in
                Text("21,000 sats")
                    .padding()
            }
        }
        .navigationTitle("Transaction History")
    }
}

struct MintView: View {
    @State var rust: ViewModel

    var body: some View {
        VStack {
            Text("Choose Mint Option")
                .font(.title)

            LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 20) {
                ForEach(["A", "B", "C", "D"], id: \.self) { option in
                    Button(action: {
                        rust.dispatch(event: .setRoute(route: .mintAmount))
                    }) {
                        Text("Mint \(option)")
                            .frame(width: 120, height: 80)
                            .background(Color.blue.opacity(0.7))
                            .foregroundColor(.white)
                            .cornerRadius(10)
                    }
                }
            }
            .padding()

            Spacer()
        }
        .navigationTitle("Mint")
    }
}

struct MintAmountView: View {
    @State var rust: ViewModel
    @State private var amount: String = "12"

    var body: some View {
        VStack {
            TextField("Amount", text: $amount)
                .padding()
                .background(Color.gray.opacity(0.2))
                .cornerRadius(8)
                .padding()

            Button(action: {
                rust.dispatch(event: .setRoute(route: .mintConfirm))
            }) {
                Text("Mint")
                    .frame(width: 100, height: 50)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Enter Amount")
    }
}

struct MintConfirmView: View {
    @State var rust: ViewModel

    var body: some View {
        VStack {
            Text("Confirm Mint")
                .font(.title)
                .padding()

            Text("12")
                .font(.largeTitle)
                .padding()

            Button(action: {
                rust.dispatch(event: .setRoute(route: .success))
            }) {
                Text("Confirm")
                    .frame(width: 120, height: 50)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Confirm")
    }
}

struct MeltView: View {
    @State var rust: ViewModel

    var body: some View {
        VStack {
            Text("Scan or paste Lightning invoice")
                .font(.title2)
                .padding()

            Rectangle()
                .fill(Color.gray.opacity(0.2))
                .frame(height: 200)
                .overlay(
                    Image(systemName: "qrcode.viewfinder")
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .frame(width: 80, height: 80)
                )

            Button(action: {
                rust.dispatch(event: .setRoute(route: .meltConfirm))
            }) {
                Text("Scan")
                    .frame(width: 120, height: 50)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Melt")
    }
}

struct MeltConfirmView: View {
    @State var rust: ViewModel

    var body: some View {
        VStack {
            Text("Send 21,000 sats to foo@bar.com?")
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            HStack(spacing: 30) {
                Button(action: {
                    rust.dispatch(event: .setRoute(route: .success))
                }) {
                    Text("Yes")
                        .frame(width: 80, height: 50)
                        .background(Color.green)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: {
                    rust.dispatch(event: .setRoute(route: .melt))
                }) {
                    Text("No")
                        .frame(width: 80, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .padding()
        }
        .navigationTitle("Confirm")
    }
}

struct SuccessView: View {
    @State var rust: ViewModel
    var message: String
    var onDismiss: () -> Void

    var body: some View {
        VStack {
            Spacer()

            Image(systemName: "party.popper.fill")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: 100, height: 100)
                .foregroundColor(.green)

            Text("Success!")
                .font(.largeTitle)
                .bold()
                .padding()

            Text(message)
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            Spacer()

            Button(action: onDismiss) {
                Text("Done")
                    .frame(width: 120, height: 50)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.bottom, 40)
        }
        .background(Color.white)
        .edgesIgnoringSafeArea(.all)
    }
}

struct ErrorView: View {
    @State var rust: ViewModel
    var error: String
    var onRetry: () -> Void
    var onQuit: () -> Void

    var body: some View {
        VStack {
            Spacer()

            Image(systemName: "exclamationmark.triangle.fill")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: 100, height: 100)
                .foregroundColor(.red)

            Text("Error")
                .font(.largeTitle)
                .bold()
                .padding()

            Text(error)
                .font(.title2)
                .multilineTextAlignment(.center)
                .padding()

            Spacer()

            HStack(spacing: 30) {
                Button(action: onRetry) {
                    Text("Retry")
                        .frame(width: 120, height: 50)
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }

                Button(action: onQuit) {
                    Text("Quit")
                        .frame(width: 120, height: 50)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .padding(.bottom, 40)
        }
        .background(Color.white)
        .edgesIgnoringSafeArea(.all)
    }
}
