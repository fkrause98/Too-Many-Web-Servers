defmodule TcpClient do
  def connect do
    start_time = :os.system_time(:millisecond)

    result =
      case :gen_tcp.connect(~c"localhost", 3000, [:binary, active: false], 5000) do
        {:ok, socket} ->
          send_request(socket)
          response = receive_response(socket)
          :gen_tcp.close(socket)
          response

        {:error, reason} ->
          {:error, "Failed to connect: #{inspect(reason)}"}
      end

    end_time = :os.system_time(:millisecond)
    {end_time - start_time, result}
  end

  defp send_request(socket) do
    request = "GET / HTTP/1.1\r\nHost: localhost:3000\r\n\r\n"
    :ok = :gen_tcp.send(socket, request)
    :ok = :gen_tcp.shutdown(socket, :write)
  end

  defp receive_response(socket) do
    case read_full_response(socket, "") do
      {:ok, response} -> {:ok, response}
      {:error, reason} -> {:error, reason}
    end
  end

  defp read_full_response(socket, acc) do
    case :gen_tcp.recv(socket, 0, 5000) do
      {:ok, data} ->
        new_acc = acc <> data

        if String.ends_with?(new_acc, "\r\n\r\n") do
          {:ok, new_acc}
        else
          read_full_response(socket, new_acc)
        end

      {:error, :closed} ->
        {:ok, acc}

      {:error, reason} ->
        {:error, reason}
    end
  end
end

defmodule Stats do
  def mean(numbers) do
    Enum.sum(numbers) / length(numbers)
  end

  def standard_deviation(numbers) do
    avg = mean(numbers)

    variance =
      Enum.reduce(numbers, 0, fn x, acc ->
        acc + :math.pow(x - avg, 2)
      end) / length(numbers)

    :math.sqrt(variance)
  end
end

# Number of requests
num_requests = 100_000

# Perform requests and collect response times
{times, results} =
  Enum.map_reduce(1..num_requests, [], fn i, acc ->
    {time, result} = TcpClient.connect()
    if rem(i, 10000) == 0, do: IO.puts("Completed #{i} requests")
    {time, [result | acc]}
  end)

# Calculate statistics
avg_time = Stats.mean(times)
std_dev = Stats.standard_deviation(times)
min_time = Enum.min(times)
max_time = Enum.max(times)
successful_requests = Enum.count(results, fn result -> elem(result, 0) == :ok end)
failed_requests = num_requests - successful_requests

# Print results
IO.puts("""
Request Summary:
Total Requests: #{num_requests}
Successful Requests: #{successful_requests}
Failed Requests: #{failed_requests}
Average Response Time: #{Float.round(avg_time, 2)} ms
Standard Deviation: #{Float.round(std_dev, 2)} ms
Minimum Response Time: #{min_time} ms
Maximum Response Time: #{max_time} ms
""")
