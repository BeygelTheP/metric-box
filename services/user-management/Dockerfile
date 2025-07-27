FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
WORKDIR /app
EXPOSE 8080

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["MetricBox.UserManagement.csproj", "./"]
RUN dotnet restore "MetricBox.UserManagement.csproj"
COPY . .
RUN dotnet build "MetricBox.UserManagement.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "MetricBox.UserManagement.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .

ENTRYPOINT ["dotnet", "MetricBox.UserManagement.dll"]
