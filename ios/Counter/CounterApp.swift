//
//  CounterApp.swift
//  Counter
//
//  Created by Justin  on 6/4/24.
//

import Counter
import SwiftUI

@main
struct CounterApp: App {
    @State var rust: ViewModel

    public init() {
        self.rust = ViewModel()
    }

    var body: some Scene {
        WindowGroup {
            AppNavigation(rust: rust)
        }
    }
}

struct MainContentView: View {
    @State var rust: ViewModel
    @State private var tabSelection: Int = 0
    @State private var showSuccess: Bool = false
    @State private var showError: Bool = false
    @State private var successMessage: String = ""
    @State private var errorMessage: String = ""

    var body: some View {
        NavigationStack {
            TabView(selection: $tabSelection) {
                HomeView(rust: rust)
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
            .onChange(of: tabSelection) { oldValue, newValue in
                if newValue == 1 {
                    rust.dispatch(event: .setRoute(route: .transactionHistory))
                } else {
                    rust.dispatch(event: .setRoute(route: .home))
                }
            }
            .fullScreenCover(isPresented: $showSuccess) {
                SuccessView(
                    rust: self.rust, message: successMessage,
                    onDismiss: {
                        showSuccess = false
                    })
            }
            .fullScreenCover(isPresented: $showError) {
                ErrorView(
                    rust: self.rust, error: errorMessage,
                    onRetry: {
                        // Handle retry logic
                        showError = false
                    },
                    onQuit: {
                        showError = false
                    })
            }
        }
        .environmentObject(rust)
        .onChange(of: rust.router.route) { oldValue, newValue in
            handleRouteChange(newValue)
        }
    }

    private func handleRouteChange(_ route: Route) {
        switch route {
        case .home:
            tabSelection = 0
        case .transactionHistory:
            tabSelection = 1
        case .success:
            successMessage = "Transaction completed successfully!"
            showSuccess = true
        case .error:
            errorMessage = "Transaction failed. Please try again."
            showError = true
        default:
            // Handle other routes if needed
            break
        }
    }
}

// MARK: - View Declarations

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
        }
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
                .keyboardType(.numberPad)
                .padding()
                .background(Color.gray.opacity(0.2))
                .cornerRadius(8)
                .padding(.horizontal)

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
